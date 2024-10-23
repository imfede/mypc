use eyre::bail;
use std::io::Read;
use std::{env, fs};

mod constants;
mod starts_with;
mod burn;
mod assemble;
mod hex_u8;

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Usage: {} <command>", args[0]);
    }

    match args[1].as_str() {
        "burn" => burn::burn(),
        "assemble" if args.len() < 3 => {
            let buffer = {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                buffer.trim().to_string()
            };
            let buffer: &'static str = buffer.leak();
            assemble::assemble::assemble(buffer)
        }
        "assemble" => {
            let file_contents = fs::read(&args[2])?;
            let file_contents = String::from_utf8(file_contents)?;
            let file_contents: &'static str = file_contents.leak();
            assemble::assemble::assemble(file_contents)
        }
        other => bail!("Unknown command: {}", other),
    }
}
