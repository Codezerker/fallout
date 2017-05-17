use warning::Hint;
use warning::Warning;

static WARNING_MATCHER: &'static str = "warning:";
static WARNING_MATCHER_UPPERCASED: &'static str = "Warning:";

pub struct Parser {
    pub warnings: Vec<Warning>,
    line_buffer: Vec<String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            warnings: vec![],
            line_buffer: vec![],
        }
    }

    pub fn parse(&mut self, lines: &mut Vec<String>) -> usize {
        self.line_buffer.append(lines);

        if self.line_buffer.len() == 0 {
            return 1;
        }

        let mut warning_buffer: Vec<Warning> = vec![];
        let mut i = 0;
        while i < self.line_buffer.len() {
            let line = self.line_at_index(i);
            if !self.is_warning(&line) {
                i += 1;
                continue;
            }

            if i + 2 < self.line_buffer.len() {
                let potential_hint = self.line_at_index(i + 2);
                if self.is_hint(&potential_hint) {
                    let source = self.line_at_index(i + 1);
                    let hint = Hint::new(source, potential_hint);
                    let warning = Warning::new(line, Some(hint));
                    warning_buffer.push(warning);
                    i += 3;
                } else {
                    let warning = Warning::new(line, None);
                    warning_buffer.push(warning);
                    i += 1;
                }
            } else {
                return 2;
            }
        }

        self.warnings.append(&mut warning_buffer);
        self.line_buffer.clear();
        return 1;
    }

    pub fn flush(&mut self) {
        // TODO: drain line_buffer
    }

    fn line_at_index(&self, index: usize) -> String {
        return self.line_buffer[index].trim_matches('\n').to_string();
    }

    fn is_warning(&self, line: &String) -> bool {
        return line.contains(WARNING_MATCHER) || line.contains(WARNING_MATCHER_UPPERCASED);
    }

    fn is_hint(&self, line: &String) -> bool {
        if line.is_empty() {
            return false;
        }
        let trimmed = line.trim_matches(|c| c == ' ' || c == '~' || c == '^');
        return trimmed.is_empty();
    }
}
