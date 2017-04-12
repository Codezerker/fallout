extern crate rustc_serialize;
mod warning;
mod parser;

use parser::Parser;
use rustc_serialize::json;
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
            println!("{:?}", json::encode(&warnings).unwrap());
        },
        Err(error) => {
            println!("{:?}", error);
        },
    };
}
