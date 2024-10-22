use std::io::Read;
use std::{env, fs};

mod constants;
mod starts_with;
mod burn;
mod assemble;
mod hex_u8;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    match args[1].as_str() {
        "burn" => burn::burn(),
        "assemble" if args.len() < 3 => {
            let buffer = {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer).unwrap();
                buffer.trim().to_string()
            };
            assemble::assemble::assemble(buffer);
        }
        "assemble" => {
            let file_contents = fs::read(&args[2]).unwrap();
            let file_contents = String::from_utf8(file_contents).unwrap();
            assemble::assemble::assemble(file_contents)
        }
        other => println!("Unknown command: {}", other),
    }
}
