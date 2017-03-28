use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // get file path
    let log_file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: fallout [xcodebuild log file]");
            process::exit(1);
        },
    };
    println!("Analysing xcodebuild log file: {:?}", log_file_path);

    // open file
    let file = match File::open(&log_file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Unable to open file: {:?}", log_file_path);
            process::exit(1);
        }
    };

    // read first line
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line);

    // print first line
    println!("");
    println!("[Debug] read file {:?} with {} lines:", log_file_path, reader.lines().count());
    println!("{}", line);
}
