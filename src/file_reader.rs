use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;

pub struct FileReader {
    io_reader: BufReader<File>,
}

impl FileReader {
    pub fn new(path: PathBuf) -> Result<FileReader, Error> {
        let file = File::open(path.as_path())?;
        let reader = BufReader::new(file);
        return Ok(FileReader { io_reader: reader });
    }

    pub fn read(&mut self, num_lines: usize) -> Vec<String> {
        let mut buffer: Vec<String> = vec![];
        for _ in 0..num_lines {
            let mut line_buffer = String::new();
            if let Ok(read_count) = self.io_reader.read_line(&mut line_buffer) {
                if read_count > 0 {
                    buffer.push(line_buffer);
                } else { // nothing read
                    break;
                }
            } else { // error happended
                break;
            }
        }
        return buffer
    }
}
