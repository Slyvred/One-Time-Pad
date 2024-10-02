use crate::helpers::{get_input, print_progress_bar, write_file};
use rand::{rngs::OsRng, TryRngCore};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, Write};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;

const BUFFER_SIZE: u64 = 512_000;
const NUM_THREADS: u64 = 4;

pub fn encrypt(file_path: &str, delete_original: bool, quiet: bool) {
    let mut path = file_path.to_string();
    if file_path.is_empty() {
        println!("Enter the path of the file to encrypt:");
        path = get_input();
    }

    // Remove quotes from path
    path = path.replace(['\"', '\''], "");

    // Check if the file exists
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            if !quiet {
                println!("File not found!");
            }
            return;
        }
    };

    // Check if there is already a pad file
    let pad_path = path.clone() + ".pad";
    if File::open(pad_path).is_ok() {
        println!("Pad file already exists!, which means the file is likely already encrypted!");
        return;
    }

    if !quiet {
        println!("Encrypting file...");
    }

    // Read file and pad by chunks
    let mut reader = BufReader::new(&file);
    let mut buffer = vec![0u8; BUFFER_SIZE as usize];

    let file_size = file.metadata().unwrap().len();

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        // Gen Pad
        let pad = gen_pad(bytes_read as u64);

        // Perform XOR with pad
        let mut encrypted_data: Vec<u8> = Vec::new();
        for i in 0..bytes_read {
            encrypted_data.push(buffer[i] ^ pad[i]);
        }

        // Append to new file
        let mut encrypted_file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.clone() + ".enc")
        {
            Ok(file) => file,
            Err(_) => {
                if !quiet {
                    println!("Failed to create/open the file!");
                }
                return;
            }
        };
        encrypted_file.write_all(&encrypted_data).unwrap();

        let mut pad_file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.clone() + ".enc.pad")
        {
            Ok(file) => file,
            Err(_) => {
                if !quiet {
                    println!("Failed to create/open the pad file!");
                }
                return;
            }
        };

        match pad_file.write_all(&pad) {
            Ok(_) => (),
            Err(_) => {
                if !quiet {
                    println!("Failed to write to the pad file!");
                }
                return;
            }
        }

        print_progress_bar(
            reader.stream_position().unwrap() as f64 / file_size as f64,
            &path,
        );
    }

    // Print newline
    println!();

    if !quiet {
        println!("File encrypted successfully!");
    }

    if delete_original {
        match std::fs::remove_file(&path) {
            Ok(_) => {
                if !quiet {
                    println!("Original file deleted!")
                }
            }
            Err(_) => {
                if !quiet {
                    println!("Failed to delete original file!")
                }
            }
        }
    }
}

pub fn encrypt_directory(directory_path: &str, delete_original: bool) {
    let mut path = directory_path.to_string();
    if directory_path.is_empty() {
        println!("Enter the path of the directory to encrypt:");
        path = get_input();
    }

    // Remove quotes from path
    path = path.replace(['\"', '\''], "");

    // Check if the folder exists
    let dir = match std::fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(_) => {
            println!("Directory not found!");
            return;
        }
    };

    let entries: Vec<_> = dir.filter_map(Result::ok).collect();
    let entries = Arc::new(Mutex::new(entries));

    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        // Number of threads
        let entries = Arc::clone(&entries);
        let handle = thread::spawn(move || {
            while let Some(entry) = entries.lock().unwrap().pop() {
                let file_path = entry.path().to_str().unwrap().to_string();

                // If it's a directory, encrypt it recursively
                if entry.file_type().unwrap().is_dir() {
                    encrypt_directory(&file_path, delete_original);
                } else {
                    encrypt(&file_path, delete_original, true);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn decrypt(file_path: &str, quiet: bool, secure_delete: bool) {
    let mut path = file_path.to_string();

    if file_path.is_empty() {
        println!("Enter the path of the file to decrypt:");
        path = get_input();
    }

    // Remove quotes from path
    path = path.replace(['\"', '\''], "");

    // Check if the file exists
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            if !quiet {
                println!("File not found!");
            }
            return;
        }
    };

    // Check if we have a pad file
    let pad_path = path.clone() + ".pad";
    let pad_file = match File::open(&pad_path) {
        Ok(file) => file,
        Err(_) => {
            if !quiet {
                println!("Pad file not found!");
            }
            return;
        }
    };

    if !quiet {
        println!("Decrypting file...");
    }

    // Read file and pad by chunks
    let mut reader = BufReader::new(&file);
    let mut buffer = vec![0u8; BUFFER_SIZE as usize];

    let mut pad_reader = BufReader::new(&pad_file);
    let mut pad_buffer = vec![0u8; BUFFER_SIZE as usize];

    let file_size = file.metadata().unwrap().len();

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        pad_reader.read_exact(&mut pad_buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        // Perform XOR with pad
        let mut decrypted_data: Vec<u8> = Vec::new();
        for i in 0..bytes_read {
            decrypted_data.push(buffer[i] ^ pad_buffer[i]);
        }

        // Append to new file

        let decrypted_file_path = path.clone().replace(".enc", "");
        let mut decrypted_file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&decrypted_file_path)
        {
            Ok(file) => file,
            Err(_) => {
                if !quiet {
                    println!("Failed to create/open the file!");
                }
                return;
            }
        };

        match decrypted_file.write_all(&decrypted_data) {
            Ok(_) => (),
            Err(_) => {
                if !quiet {
                    println!("Failed to write to the file!");
                }
                return;
            }
        }

        print_progress_bar(
            reader.stream_position().unwrap() as f64 / file_size as f64,
            &path,
        );
    }

    // Print newline
    println!();

    if !quiet {
        println!("File decrypted successfully!");
    }

    // Fill pad with zeros, as fs::remove_file does not actually delete the file depending on the platform
    if secure_delete {
        if !quiet {
            println!("Filling pad with zeros...");
        }

        let zeros = vec![0u8; pad_file.metadata().unwrap().len() as usize];
        write_file(&zeros, &pad_path);
    }

    // Delete pad file
    match std::fs::remove_file(&pad_path) {
        Ok(_) => {
            if !quiet {
                println!("Pad file deleted! ")
            }
        }
        Err(_) => {
            if !quiet {
                println!("Failed to delete pad file!")
            }
        }
    }

    // Delete encrypted file
    match std::fs::remove_file(&path) {
        Ok(_) => {
            if !quiet {
                println!("Encrypted file deleted!")
            }
        }
        Err(_) => {
            if !quiet {
                println!("Failed to delete encrypted file!")
            }
        }
    }
}

pub fn decrypt_directory(directory_path: &str, secure_delete: bool) {
    let mut path = directory_path.to_string();
    if directory_path.is_empty() {
        println!("Enter the path of the directory to decrypt:");
        path = get_input();
    }

    // Remove quotes from path
    path = path.replace(['\"', '\''], "");

    // Check if the folder exists
    let dir = match std::fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(_) => {
            println!("Directory not found!");
            return;
        }
    };

    let entries: Vec<_> = dir.filter_map(Result::ok).collect();
    let entries = Arc::new(Mutex::new(entries));

    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        // Number of threads
        let entries = Arc::clone(&entries);
        let handle = thread::spawn(move || {
            while let Some(entry) = entries.lock().unwrap().pop() {
                let file_path = entry.path().to_str().unwrap().to_string();

                // If it's a directory, encrypt it recursively
                if entry.file_type().unwrap().is_dir() {
                    decrypt_directory(&file_path, secure_delete);
                } else {
                    decrypt(&file_path, true, secure_delete);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn gen_pad(file_size: u64) -> Vec<u8> {
    let mut pad = vec![0u8; file_size as usize];
    match OsRng.try_fill_bytes(&mut pad) {
        Ok(_) => pad,
        Err(_) => {
            println!("Failed to generate pad!");
            exit(1);
        }
    }
}
