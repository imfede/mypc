use std::env;

mod burner;
mod constants;
mod starts_with;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    match args[1].as_str() {
        "burn" => burner::burn(),
        other => println!("Unknown command: {}", other),
    }
}
