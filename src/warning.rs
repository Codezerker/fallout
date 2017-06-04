use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Warning {
    pub message: String,
    pub hint: Option<Hint>,
    pub location: Option<Location>,
}

impl Warning {
    pub fn new(line: String, hint: Option<Hint>, location: Option<Location>) -> Warning {
        Warning {
            message: line,
            hint: hint,
            location: location,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub line_num: u64,
    pub colum_num: u64,
    pub file_path: String,
    pub module_name: String,
}

impl Location {
    pub fn new(line_num: u64, colum_num: u64, file_path: String, module_name: String) -> Location {
        Location{
            line_num: line_num,
            colum_num: colum_num,
            file_path: file_path,
            module_name: module_name,
        }
    }
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
