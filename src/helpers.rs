use crate::xor::{decrypt, encrypt};
use std::fs::File;
use std::io;
use std::io::{/*BufReader, Read,*/ Write};
use std::process::exit;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

pub fn display_menu() {
    println!("1. Encrypt");
    println!("2. Decrypt");
    println!("3. Exit");

    println!("Enter your choice: ");
    let choice = get_input();

    let choice: i32 = match choice.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice!");
            display_menu();
            return;
        }
    };

    match choice {
        1 => {
            println!("Do you want to delete the original file after encryption? (y/n)");
            let delete_original = get_input();
            match delete_original.as_str() {
                "y" => encrypt("", true, false),
                "n" => encrypt("", false, false),
                _ => {
                    println!("Invalid choice!");
                    display_menu();
                }
            }
        }
        2 => {
            println!("Do you want to securely delete the original file after decryption? (y/n)");
            let secure_delete = get_input();
            match secure_delete.as_str() {
                "y" => decrypt("", false, true),
                "n" => decrypt("", false, false),
                _ => {
                    println!("Invalid choice!");
                    display_menu();
                }
            }
        }
        3 => exit(0),
        _ => {
            println!("Invalid choice!");
            display_menu();
        }
    }
}

pub fn write_file(data: &[u8], path: &String) {
    // Check if the file exists / if we can open it
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to create/open the file!");
            exit(1);
        }
    };

    match file.write_all(data) {
        Ok(_) => (),
        Err(_) => {
            println!("Failed to write to the file!");
            exit(1);
        }
    }
}

pub fn print_progress_bar(progress: f64, filename: &str) {
    let bar_width = 32;
    let pos = (bar_width as f64 * progress).round() as usize;
    let bar: String = (0..bar_width)
        .map(|i| if i < pos { '=' } else { ' ' })
        .collect();
    print!("\r[{}] {}", bar, filename);
    std::io::stdout().flush().unwrap();
}
