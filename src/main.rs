use std::env;
use std::process;

mod warning;
mod parser;

use parser::Parser;

fn main() {
    // get file path
    let log_file = match env::args().nth(1) {
        Some(file) => file,
        None => {
            println!("Usage: fallout [xcodebuild log file]");
            process::exit(1);
        },
    };
    println!("Analysing xcodebuild log file: {:?}", log_file);

    // parse file
    let parser = Parser::new(log_file);
}
