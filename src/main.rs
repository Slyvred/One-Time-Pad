use std::env;

mod helpers;
mod xor;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        helpers::display_menu();
        return;
    }

    let path = args.iter().find(|&arg| !arg.starts_with("--")).expect("File path not provided");
    let delete = args.contains(&"--delete".to_string());
    let mode = args.iter().find(|&arg| arg == "--encrypt" || arg == "--decrypt").expect("Invalid mode");
    let dir = args.contains(&"--dir".to_string());
    let secure_delete = args.contains(&"--secure".to_string());

    match mode.as_str() {
        "--encrypt" => {
            if dir {
                xor::encrypt_directory(path, delete);
            } else {
                xor::encrypt(path, delete, false);
            }
        },
        "--decrypt" => {
            if dir {
                xor::decrypt_directory(path, secure_delete);
            } else {
                xor::decrypt(path, false, secure_delete);
            }
        },
        _ => {
            println!("Invalid arguments!");
            println!("Usage: ./one_time_pad --encrypt <file> | --decrypt <file>");
            println!("Options:");
            println!("\t--delete: Delete the original file after encryption");
            println!("\t--dir: Encrypt/decrypt a directory");
            println!("\t--secure: Securely delete the original file after decryption (pad file gets filled with zeros)");
        }
    }
}