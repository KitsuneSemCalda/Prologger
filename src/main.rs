mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <prolog file path>", args[0]);
        std::process::exit(1);
    }

    let filepath = &args[1];

    if let Err(e) = parser::utils::parse_file(filepath) {
        eprintln!("File to process file: {}", e);
    }
}
