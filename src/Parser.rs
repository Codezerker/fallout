use std::string::String;
use std::io::Error;
use warning::Warning;

static WARNING_MATCHER: &'static str = "warning:";

pub struct Parser {
    pub path: String,
}

impl Parser {
    pub fn new(path: String) -> Parser {
        Parser { path: path }
    }

    pub fn parse(&self) /* -> Result<Vec<Warning>, Error> */ {
        // ...
    }
}

/*
// open file
let file = match File::open(&log_file_path) {
    Ok(file) => file,
    Err(_) => {
        println!("Unable to open file: {:?}", log_file_path);
        process::exit(1);
    }
};

// read lines
let reader = BufReader::new(file);
for line in reader.lines() {
    let valid_line = match line {
        Ok(l) => l,
        Err(e) => {
            println!("Unexpected error when reading file: {:?}", e);
            process::exit(1);
        },
    };
    if valid_line.contains(WARNING_MATCHER) {
        println!("{}", &valid_line);
    }
}
*/
