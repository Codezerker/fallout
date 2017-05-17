use std::path::PathBuf;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Warning {
    pub message: String,
    pub hint: Option<Hint>,
    pub location: Option<Location>,
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
    pub line: u64,
    pub column: u64,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hint {
    pub source: String,
    pub indicator: String,
}

impl Hint {
    pub fn new(source: String, indicator: String) -> Hint {
        Hint {
            source: source,
            indicator: indicator,
        }
    }
}
