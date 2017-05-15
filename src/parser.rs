use warning::Hint;
use warning::Warning;

static WARNING_MATCHER: &'static str = "warning:";

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

        let mut i = 0;
        while i < self.line_buffer.len() {
            let line = self.line_buffer[i].clone();
            if !line.contains(WARNING_MATCHER) {
                i += 1;
                continue;
            }

            if i + 2 < self.line_buffer.len() {
                let potential_hint = self.line_buffer[i + 2].clone();
                if self.is_hint(&potential_hint) {
                    let source = self.line_buffer[i + 1].clone();
                    let hint = Hint::new(source, potential_hint);
                    let warning = Warning::new(line, Some(hint));
                    self.warnings.push(warning);
                    i += 2;
                } else {
                    let warning = Warning::new(line, None);
                    self.warnings.push(warning);
                    i += 1;
                }
            } else {
                return 2;
            }
        }

        self.line_buffer.clear();
        return 1;
    }

    pub fn flush(&mut self) {
        // TODO: drain line_buffer
    }

    fn is_hint(&self, line: &String) -> bool {
        let trimmed = line.trim_matches(|c| c == ' ' || c == '~' || c == '^');
        return trimmed == "\n";
    }
}
