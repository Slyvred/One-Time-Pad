use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::process::exit;
use crate::xor::{decrypt, encrypt};

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

pub fn display_menu() {
    println!("1. Encrypt");
    println!("2. Decrypt");
    println!("3. Exit");

    println!("Enter your choice:");
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
        1 => encrypt(),
        2 => decrypt(),
        3 => exit(0),
        _ => {
            println!("Invalid choice!");
            display_menu();
        }
    }
}

pub fn write_file(data: Vec<u8>, path: &String) {
    let mut file = File::create(path).unwrap();
    file.write_all(&data).unwrap();
}

pub fn read_file(path: &String) -> Vec<u8> {
    let mut buf = BufReader::new(File::open(path).unwrap());

    let mut data: Vec<u8> = Vec::new();

    match buf.read_to_end(&mut data) {
        Ok(_) => data,
        Err(_) => {
            println!("Failed to read file!");
            Vec::new()
        }
    }
}