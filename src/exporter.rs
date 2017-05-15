use serde_json;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use warning::Warning;

static DEFAULT_OUTPUT_PATH: &'static str = "./xcodebuild_warnings.json";

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => error.fmt(f),
            Error::Json(ref error) => error.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref error) => error.description(),
            Error::Json(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::Json(ref error) => Some(error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::Json(error)
    }
}

pub struct Exporter {}

impl Exporter {
    pub fn new() -> Exporter {
        Exporter {}
    }

    pub fn export(&self, warnings: &Vec<Warning>) -> Result<(), Error> {
        println!("");
        println!("=== Exporting report to: {} ===", DEFAULT_OUTPUT_PATH);
        println!("");

        let file = File::create(DEFAULT_OUTPUT_PATH)?;
        let _ = serde_json::to_writer_pretty(file, warnings)?;
        return Ok(());
    }
}
