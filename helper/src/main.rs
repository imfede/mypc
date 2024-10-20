use std::env;

mod constants;
mod starts_with;
mod burn;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    match args[1].as_str() {
        "burn" => burn::burn(),
        other => println!("Unknown command: {}", other),
    }
}
