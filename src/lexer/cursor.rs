use super::Lexer;

impl Lexer {
    pub(crate) fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    pub(crate) fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    pub(crate) fn peek_char_at(&self, offset: usize) -> Option<char> {
        if self.position + offset < self.input.len() {
            Some(self.input[self.position + offset])
        } else {
            None
        }
    }

    pub(crate) fn advance(&mut self) {
        if self.position < self.input.len() {
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    pub(crate) fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub(crate) fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    pub(crate) fn skip_multiline_comment(&mut self) -> Result<(), String> {
        let start_line = self.line;
        let start_col = self.column;
        while let Some(ch) = self.current_char() {
            if ch == '*' && self.peek_char() == Some('/') {
                self.advance(); // skip '*'
                self.advance(); // skip '/'
                return Ok(());
            }
            self.advance();
        }
        Err(format!(
            "Unclosed multiline comment starting at line {}, column {}",
            start_line, start_col
        ))
    }
}
