use xcode_output::file_reader::FileReader;
use xcode_output::parser::Parser;
use xcode_output::warning::Warning;
use std::io::Error;
use std::path::PathBuf;

pub struct Driver {
    file_reader: FileReader,
    parser: Parser,
}

impl Driver {
    pub fn new(file_path: PathBuf) -> Result<Driver, Error> {
        let file_reader = FileReader::new(file_path)?;
        let parser = Parser::new();
        return Ok(Driver {
            file_reader: file_reader,
            parser: parser,
        });
    }

    pub fn run(&mut self) {
        let mut lines = self.file_reader.read(1);
        while lines.len() > 0 {
            let num_lines = self.parser.parse(&mut lines);
            lines = self.file_reader.read(num_lines);
        }
        self.parser.flush();
    }

    pub fn parsed_warnings(&mut self) -> &mut Vec<Warning> {
        return &mut self.parser.warnings;
    }
}
