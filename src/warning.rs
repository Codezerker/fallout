use std::path::PathBuf;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Warning {
    message: String,
    hint: Option<Hint>,
    location: Option<Location>,
}

impl Warning {
    pub fn new(line: String, hint: Option<Hint>) -> Warning {
        Warning {
            message: line,
            hint: hint,
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    line: u64,
    column: u64,
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hint {
    source: String,
    indicator: String,
}

impl Hint {
    pub fn new(source: String, indicator: String) -> Hint {
        Hint {
            source: source,
            indicator: indicator,
        }
    }
}
