use std::env;

mod helpers;
mod xor;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let mode = &args[1];
            let file = &args[2];

            match mode.as_str() {
                "--encrypt" => xor::encrypt(file),
                "--decrypt" => xor::decrypt(file),
                _ => println!("Invalid mode"),
            }
        }
        1 => {
            helpers::display_menu();
        }
        _ => {
            println!("Invalid arguments!");
            print!("Usage: ");
            println!("./one_time_pad --encrypt <file> | --decrypt <file>");
        }
    }
}
