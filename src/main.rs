use std::env;

mod helpers;
mod xor;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // Remove the first argument because we find the path by checking for the first argument that doesn't start with "--"

    match args.len() {
        0 => helpers::display_menu(),
        _ => {
            let path = args.iter().find(|&arg| !arg.starts_with("--"));

            if args.iter().find(|&arg| arg == "--encrypt").is_some() {
                // Check if the user has provided a delete flag
                xor::encrypt(path.unwrap().as_str(), args.iter().find(|&arg| arg == "--delete").is_some());
            }
            else if args.iter().find(|&arg| arg == "--decrypt").is_some() {
                xor::decrypt(path.unwrap().as_str());
            }
            else {
                println!("Invalid arguments!");
                println!("Usage: ./one_time_pad --encrypt (--delete) <file> | --decrypt <file>");
            }
        }
    }
}
