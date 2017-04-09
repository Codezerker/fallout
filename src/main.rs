use std::env;
use std::path::PathBuf;
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

    // parse file
    let file_path = PathBuf::from(log_file);
    let parser = Parser::new(file_path);
    parser.parse();
}
