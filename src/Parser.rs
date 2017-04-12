use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;
use std::process;
use std::string::String;
use warning::Warning;

static WARNING_MATCHER: &'static str = "warning:";

pub struct Parser {
    pub path: PathBuf,
}

impl Parser {
    pub fn new(path: PathBuf) -> Parser {
        Parser { path: path }
    }

    pub fn parse(&self) -> Result<Vec<Warning>, Error> {
        println!("");
        println!("=== Analysing xcodebuild log at: {:?} ===", self.path.as_path());
        println!("");

        let file = match File::open(self.path.as_path()) {
            Ok(file) => file,
            Err(error) => {
                return Err(error);
            }
        };

        let reader = BufReader::new(file);
        let mut warnings: Vec<Warning> = vec![];
        for (index, line) in reader.lines().enumerate() {
            let unwraped_line = line.unwrap();
            if unwraped_line.contains(WARNING_MATCHER) {
                warnings.push(Warning::new(unwraped_line));
            }
        }
        return Ok(warnings);
    }
}
