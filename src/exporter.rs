use serde_json;
use std::fs::File;
use warning::Warning;

static DEFAULT_OUTPUT_PATH: &'static str = "./xcodebuild_warnings.json";

pub struct Exporter {}

impl Exporter {
    pub fn new() -> Exporter {
        Exporter {}
    }

    pub fn export(&self, warnings: Vec<Warning>) {
        println!("");
        println!("=== Exporting report to: {} ===", DEFAULT_OUTPUT_PATH);
        println!("");

        // TODO: pop errors
        let mut file = File::create(DEFAULT_OUTPUT_PATH).unwrap();
        let writer = serde_json::to_writer_pretty(file, &warnings).unwrap();
    }
}
