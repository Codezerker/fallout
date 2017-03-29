use std::path::Path;
use std::string::String;

pub struct Warning {
    message: String,
    hint: Hint,
    location: Location,
}

pub struct Location {
    line: u64,
    column: u64,
    path: Path,
}

pub struct Hint {
    source: String,
    indicator: String,
}
