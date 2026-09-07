mod cursor;
mod reader;
#[cfg(test)]
mod tests;
pub mod token;

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
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::PlusAssign,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Plus,
                            line,
                            column,
                        });
                    }
                }
                '-' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::MinusAssign,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Minus,
                            line,
                            column,
                        });
                    }
                }
                '*' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::MultiplyAssign,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Multiply,
                            line,
                            column,
                        });
                    }
                }
                '/' => {
                    self.advance();
                    if self.current_char() == Some('/') {
                        self.skip_comment();
                    } else if self.current_char() == Some('*') {
                        self.advance();
                        self.skip_multiline_comment()?;
                    } else if self.current_char() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::DivideAssign,
                            line,
                            column,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Divide,
                            line,
                            column,
                        });
                    }
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
                '[' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::LeftBracket,
                        line,
                        column,
                    });
                }
                ']' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::RightBracket,
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
                ':' => {
                    self.advance();
                    tokens.push(Token {
                        token_type: TokenType::Colon,
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
                    } else if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                        let (num, _) = self.read_number()?;
                        tokens.push(Token {
                            token_type: TokenType::Float(num),
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
                        tokens.push(Token {
                            token_type: TokenType::Not,
                            line,
                            column,
                        });
                    }
                }
                '&' => {
                    self.advance();
                    if self.current_char() == Some('&') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::And,
                            line,
                            column,
                        });
                    } else {
                        return Err(format!(
                            "Unexpected character '&' at line {}, column {}. Did you mean '&&'?",
                            line, column
                        ));
                    }
                }
                '|' => {
                    self.advance();
                    if self.current_char() == Some('|') {
                        self.advance();
                        tokens.push(Token {
                            token_type: TokenType::Or,
                            line,
                            column,
                        });
                    } else {
                        return Err(format!(
                            "Unexpected character '|' at line {}, column {}. Did you mean '||'?",
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
                    let (num, is_float) = self.read_number()?;
                    let token_type = if is_float {
                        TokenType::Float(num)
                    } else {
                        TokenType::Number(num)
                    };
                    tokens.push(Token {
                        token_type,
                        line,
                        column,
                    });
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    let token_type = TokenType::from_identifier(&ident);
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
