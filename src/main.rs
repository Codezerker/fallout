extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod warning;
mod parser;

use parser::Parser;
use std::env;
use std::path::PathBuf;
use std::process;

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
    match parser.parse() {
        Ok(warnings) => {
            println!("Number of warnings: {}", warnings.len());

            println!("");
            println!("{}", serde_json::to_string_pretty(&warnings).unwrap());
        },
        Err(error) => {
            println!("{:?}", error);
        },
    };
}
