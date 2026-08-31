use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("start") => println!("grip start"),
        Some("status") => println!("grip status"),
        Some("snap") => println!("grip snap"),
        Some("trace") => println!("grip trace"),
        Some("undo") => println!("grip undo"),
        Some("help") | None => help(),
        Some(command) => {
            eprintln!("Unknown command: {}", command);
            help();
        }
    }
}

fn help() {
    println!("Grip 0.1.0");
    println!();
    println!("Usage:");
    println!("  grip start");
    println!("  grip status");
    println!("  grip snap <message>");
    println!("  grip trace");
    println!("  grip undo");
}
