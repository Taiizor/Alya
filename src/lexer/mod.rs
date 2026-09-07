pub mod token;
#[cfg(test)]
mod tests;

pub use token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let start_pos = self.position;
        let mut has_dot = false;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot && self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let num_str: String = self.input[start_pos..self.position].iter().collect();
        num_str
            .parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", num_str))
    }

    fn read_string(&mut self) -> Result<String, String> {
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
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('{') => result.push('{'),
                    Some('}') => result.push('}'),
                    Some(c) => result.push(c),
                    None => return Err("Unexpected end of string".to_string()),
                }
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err("Unterminated string".to_string())
    }

    fn read_identifier(&mut self) -> String {
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char() {
            let line = self.line;
            let column = self.column;

            match ch {
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace();
                }
                '\n' => {
                    tokens.push(Token {
                        token_type: TokenType::Newline,
                        line,
                        column,
                    });
                    self.advance();
                }
                '#' => {
                    self.skip_comment();
                }
                '"' => {
                    let s = self.read_string()?;
                    tokens.push(Token {
                        token_type: TokenType::String(s),
                        line,
                        column,
                    });
                }
                '+' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Plus,
                        line,
                        column,
                    });
                }
                '-' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Minus,
                        line,
                        column,
                    });
                }
                '*' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Multiply,
                        line,
                        column,
                    });
                }
                '/' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Divide,
                        line,
                        column,
                    });
                }
                '%' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Modulo,
                        line,
                        column,
                    });
                }
                '(' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::LeftParen,
                        line,
                        column,
                    });
                }
                ')' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::RightParen,
                        line,
                        column,
                    });
                }
                '{' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::LeftBrace,
                        line,
                        column,
                    });
                }
                '}' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::RightBrace,
                        line,
                        column,
                    });
                }
                ',' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Comma,
                        line,
                        column,
                    });
                }
                '.' => {
                    if self.peek_char() == Some('.') {
                        self.advance();
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::DotDot,
                            line,
                            column,
                        });
                    } else {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::Dot,
                            line,
                            column,
                        });
                    }
                }
                '=' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::Equal,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Assign,
                            line,
                            column,
                        });
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::NotEqual,
                            line,
                            column,
                        });
                    } else {
                        return Err(format!(
                            "Unexpected character '!' at line {}, column {}",
                            line, column
                        ));
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::LessEqual,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Less,
                            line,
                            column,
                        });
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::GreaterEqual,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Greater,
                            line,
                            column,
                        });
                    }
                }
                _ if ch.is_ascii_digit() => {
                    let num = self.read_number()?;
                    tokens.push(Token {
                        token_type: TokenType::Number(num),
                        line,
                        column,
                    });
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    let token_type = match ident.as_str() {
                        "say" => TokenType::Say,
                        "let" => TokenType::Let,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "for" => TokenType::For,
                        "in" => TokenType::In,
                        "function" => TokenType::Function,
                        "end" => TokenType::End,
                        "return" => TokenType::Return,
                        "when" => TokenType::When,
                        "is" => TokenType::Is,
                        "then" => TokenType::Then,
                        "repeat" => TokenType::Repeat,
                        "break" => TokenType::Break,
                        "continue" => TokenType::Continue,
                        "ask" => TokenType::Ask,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "and" => TokenType::And,
                        "or" => TokenType::Or,
                        "not" => TokenType::Not,
                        _ => TokenType::Identifier(ident),
                    };
                    tokens.push(Token {
                        token_type,
                        line,
                        column,
                    });
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        ch, line, column
                    ));
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }
}
