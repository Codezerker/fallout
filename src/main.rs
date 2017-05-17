extern crate colored;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod driver;
mod exporter;
mod file_reader;
mod parser;
mod warning;

use colored::*;
use driver::Driver;
use exporter::Exporter;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process;
use warning::Warning;

fn main() {
    let file_path = get_log_file_path();

    println!("\n{} {} {}\n", "=== Analyzing:".blue(),
                             &file_path.to_str().unwrap().magenta(),
                             "===".blue());

    let mut driver = match Driver::new(file_path) {
        Ok(driver) => driver,
        Err(error) => {
            println!("Error: {}", error.description());
            process::exit(1);
        },
    };
    driver.run();
    let warnings = driver.parsed_warnings();
    export_parsed_warnings_as_json(warnings);
}

fn get_log_file_path() -> PathBuf {
    let log_file = match env::args().nth(1) {
        Some(file) => file,
        None => {
            println!("Usage: fallout [xcodebuild log file]");
            process::exit(1);
        },
    };
    return PathBuf::from(log_file);
}

fn export_parsed_warnings_as_json(warnings: &Vec<Warning>) {
    println!("Number of warnings: {}", warnings.len().to_string().yellow());

    let exporter = Exporter::new();
    match exporter.export(warnings) {
        Ok(_) => {},
        Err(error) => {
            println!("Error: {}", error.description().red());
        },
    };
    println!("{}\n", "SUCCESS".green());
}
