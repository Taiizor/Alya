use super::Lexer;

impl Lexer {
    pub(crate) fn read_number(&mut self) -> Result<(f64, bool), String> {
        let start_pos = self.position;
        let start_line = self.line;
        let start_col = self.column;
        let mut has_dot = false;

        if self.current_char() == Some('.') {
            has_dot = true;
            self.advance();
        }

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot && self.peek_char().is_some_and(|c| c.is_ascii_digit())
            {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let num_str: String = self.input[start_pos..self.position].iter().collect();
        let val = num_str.parse::<f64>().map_err(|_| {
            format!(
                "Invalid number '{}' at line {}, column {}",
                num_str, start_line, start_col
            )
        })?;
        Ok((val, has_dot))
    }

    pub(crate) fn read_string(&mut self) -> Result<String, String> {
        let start_line = self.line;
        let start_col = self.column;
        self.advance(); // Skip opening quote
        let mut result = String::new();

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(result);
            } else if ch == '\\' {
                self.advance();
                match self.current_char() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('e') => result.push('\x1b'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('{') => result.push('{'),
                    Some('}') => result.push('}'),
                    Some(c) => result.push(c),
                    None => {
                        return Err(format!(
                            "Unexpected end of string at line {}, column {}",
                            self.line, self.column
                        ))
                    }
                }
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err(format!(
            "Unterminated string starting at line {}, column {}",
            start_line, start_col
        ))
    }

    pub(crate) fn read_triple_quoted_string(&mut self) -> Result<String, String> {
        let start_line = self.line;
        let start_col = self.column;
        self.advance();
        self.advance();
        self.advance();

        if self.current_char() == Some('\r') && self.peek_char() == Some('\n') {
            self.advance();
            self.advance();
        } else if self.current_char() == Some('\n') {
            self.advance();
        }

        let mut result = String::new();
        while let Some(ch) = self.current_char() {
            if ch == '"' && self.peek_char() == Some('"') && self.peek_char_at(2) == Some('"') {
                self.advance();
                self.advance();
                self.advance();
                return Ok(result);
            } else if ch == '\r' {
                self.advance();
                if self.current_char() == Some('\n') {
                    self.advance();
                }
                result.push('\n');
            } else if ch == '\\' {
                self.advance();
                match self.current_char() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('e') => result.push('\x1b'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('{') => result.push('{'),
                    Some('}') => result.push('}'),
                    Some(c) => {
                        result.push('\\');
                        result.push(c);
                    }
                    None => {
                        return Err(format!(
                            "Unexpected end of string at line {}, column {}",
                            self.line, self.column
                        ));
                    }
                }
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err(format!(
            "Unterminated multiline string starting at line {}, column {}",
            start_line, start_col
        ))
    }

    pub(crate) fn read_raw_string(&mut self) -> Result<String, String> {
        let start_line = self.line;
        let start_col = self.column;
        self.advance(); // Skip opening `
        let mut result = String::new();

        while let Some(ch) = self.current_char() {
            if ch == '`' {
                self.advance(); // Skip closing `
                return Ok(result);
            } else if ch == '\r' {
                self.advance();
                if self.current_char() == Some('\n') {
                    self.advance();
                }
                result.push('\n');
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err(format!(
            "Unterminated raw string starting at line {}, column {}",
            start_line, start_col
        ))
    }

    pub(crate) fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start_pos..self.position].iter().collect()
    }
}
