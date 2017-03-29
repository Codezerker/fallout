use std::string::String;
use std::io::Error;
use warning::Warning;

static WARNING_MATCHER: &'static str = "warning:";

pub struct Parser {
    path: String,
}

impl Parser {
    pub fn new(path: String) -> Parser {
        Parser { path: path }
    }

    pub fn parse() -> Result<Vec<Warning>, Error> {
        // ...
    }
}
