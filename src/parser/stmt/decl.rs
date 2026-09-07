use crate::ast::*;
use crate::lexer::TokenType;
use crate::parser::Parser;

impl Parser {
    pub(super) fn parse_import(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'import'

        let path = match &self.current_token().token_type {
            TokenType::String(s) => s.clone(),
            _ => {
                return Err(format!(
                    "Expected string literal after 'import' at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ))
            }
        };
        self.advance();

        Ok(Stmt::Import(path))
    }

    pub(super) fn parse_say(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'say'
        let expr = self.parse_expression()?;
        Ok(Stmt::Say(expr))
    }

    pub(super) fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'let'

        let name = match &self.current_token().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => {
                return Err(format!(
                    "Expected identifier after 'let' at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ))
            }
        };
        self.advance();

        self.expect(TokenType::Assign)?;
        let value = self.parse_expression()?;

        Ok(Stmt::Let { name, value })
    }

    pub(super) fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'function'

        let name = match &self.current_token().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => {
                return Err(format!(
                    "Expected function name at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ))
            }
        };
        self.advance();

        self.expect(TokenType::LeftParen)?;

        let mut params = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::RightParen) {
            if let TokenType::Identifier(s) = &self.current_token().token_type {
                params.push(s.clone());
                self.advance();

                if matches!(self.current_token().token_type, TokenType::Comma) {
                    self.advance();
                }
            } else {
                return Err(format!(
                    "Expected parameter name at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ));
            }
        }

        self.expect(TokenType::RightParen)?;
        self.skip_newlines();

        let mut body = Vec::new();
        while !matches!(
            self.current_token().token_type,
            TokenType::End | TokenType::Eof
        ) {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(TokenType::End)?;

        Ok(Stmt::Function { name, params, body })
    }

    pub(super) fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'return'

        if matches!(
            self.current_token().token_type,
            TokenType::Newline | TokenType::Eof
        ) {
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expression()?;
            Ok(Stmt::Return(Some(expr)))
        }
    }

    pub(super) fn parse_struct(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'struct'

        let name = match &self.current_token().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => {
                return Err(format!(
                    "Expected struct name at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ))
            }
        };
        self.advance();
        self.skip_newlines();

        let mut fields = Vec::new();
        while !matches!(
            self.current_token().token_type,
            TokenType::End | TokenType::Eof
        ) {
            self.skip_newlines();
            if matches!(
                self.current_token().token_type,
                TokenType::End | TokenType::Eof
            ) {
                break;
            }
            match &self.current_token().token_type {
                TokenType::Identifier(field) => {
                    fields.push(field.clone());
                    self.advance();
                    if matches!(self.current_token().token_type, TokenType::Comma) {
                        self.advance();
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected field name or 'end' at line {}, column {}",
                        self.current_token().line,
                        self.current_token().column
                    ))
                }
            }
            self.skip_newlines();
        }

        self.expect(TokenType::End)?;

        Ok(Stmt::StructDef { name, fields })
    }
}
