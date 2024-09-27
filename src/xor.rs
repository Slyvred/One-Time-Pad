use crate::helpers::{get_input, read_file, write_file};
use rand::{rngs::OsRng, TryRngCore};
use std::fs::File;

pub fn encrypt(file: &str) {
    let mut path = file.to_string();
    if file.is_empty() {
        println!("Enter the path of the file to encrypt:");
        path = get_input();
    }

    // Check if the file exists
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("File not found!");
            return;
        }
    };

    // Check if there is already a pad file
    let pad_path = path.clone() + ".pad";
    if File::open(&pad_path).is_ok() {
        println!("Pad file already exists!, which means the file is likely already encrypted!");
        return;
    }

    // Gen Pad
    println!("Generating pad...");
    let file_size = file.metadata().unwrap().len();
    let pad = gen_pad(file_size);

    // Read file
    println!("Reading file...");
    let data = read_file(&path);

    // Perform XOR with pad
    println!("Encrypting file...");
    let mut encrypted_data: Vec<u8> = Vec::new();
    for i in 0..data.len() {
        encrypted_data.push(data[i] ^ pad[i]);
    }

    // Write to file
    write_file(encrypted_data, &path);

    // Write pad to file
    let pad_path = path + ".pad";
    write_file(pad, &pad_path);

    println!("File encrypted successfully!");
}

pub fn gen_pad(file_size: u64) -> Vec<u8> {
    let mut pad = vec![0u8; file_size as usize];
    match OsRng.try_fill_bytes(&mut pad) {
        Ok(_) => pad,
        Err(_) => {
            println!("Failed to generate pad!");
            Vec::new()
        }
    }
}

pub fn decrypt(file: &str) {
    let mut path = file.to_string();
    if file.is_empty() {
        println!("Enter the path of the file to decrypt:");
        path = get_input();
    }

    // Check if the file exists
    let _ = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("File not found!");
            return;
        }
    };

    // Read file
    println!("Reading file...");
    let data = read_file(&path);

    // Read pad
    let pad_path = path.clone() + ".pad";

    // Check if the file exists
    let _ = match File::open(&pad_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Pad not found!");
            return;
        }
    };

    let pad = read_file(&pad_path);

    // Perform XOR with pad
    println!("Decrypting file...");
    let mut decrypted_data: Vec<u8> = Vec::new();
    for i in 0..data.len() {
        decrypted_data.push(data[i] ^ pad[i]);
    }

    // Write to file
    write_file(decrypted_data, &path);
    println!("File decrypted successfully!");

    // Delete pad file
    match std::fs::remove_file(&pad_path) {
        Ok(_) => (),
        Err(_) => println!("Failed to delete pad file!"),
    }
}
