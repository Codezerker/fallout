use std::env;
use std::process;
use std::error::Error;
use std::path::PathBuf;

extern crate colored;
use colored::*;

extern crate fallout;
use fallout::driver::Driver;
use fallout::exporter::Exporter;

static DEFAULT_OUTPUT_PATH: &'static str = "./xcodebuild_warnings.json";

fn main() {
    let log_file = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: fallout [xcodebuild log file]");
        process::exit(1);
    });
    let file_path = PathBuf::from(log_file);

    println!("\n{} {} {}\n", "=== Analyzing:".blue(),
                             &file_path.to_str().unwrap().magenta(),
                             "===".blue());

    let mut driver = Driver::new(file_path).unwrap_or_else(|error| {
        println!("Error: {}\n", error.description().red());
        process::exit(1);
    }); 
    driver.run();
    let warnings = driver.parsed_warnings();

    println!("Number of warnings: {}", warnings.len().to_string().yellow());
    println!("\n{} {} {}\n", "=== Exporting report to:".blue(),
                             DEFAULT_OUTPUT_PATH.magenta(),
                             "===".blue());

    Exporter::new().export(warnings, DEFAULT_OUTPUT_PATH).unwrap_or_else(|error| {
        println!("Error: {}\n", error.description().red());
        process::exit(1);
    });

    println!("{}\n", "SUCCESS".green());
}
