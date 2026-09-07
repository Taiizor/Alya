use super::Parser;
use crate::ast::*;
use crate::lexer::TokenType;

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();

        match &self.current_token().token_type {
            TokenType::Say => self.parse_say(),
            TokenType::Let => self.parse_let(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Function => self.parse_function(),
            TokenType::Return => self.parse_return(),
            TokenType::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            TokenType::Continue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            TokenType::When => self.parse_when(),
            TokenType::Try => self.parse_try_catch(),
            TokenType::Identifier(_) => {
                // Could be assignment or function call
                let ident = match &self.current_token().token_type {
                    TokenType::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };
                self.advance();

                match self.current_token().token_type {
                    TokenType::Assign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(Stmt::Assign { name: ident, value })
                    }
                    TokenType::PlusAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(Stmt::Assign {
                            name: ident.clone(),
                            value: Expr::Binary {
                                left: Box::new(Expr::Identifier(ident)),
                                op: BinaryOp::Add,
                                right: Box::new(value),
                            },
                        })
                    }
                    TokenType::MinusAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(Stmt::Assign {
                            name: ident.clone(),
                            value: Expr::Binary {
                                left: Box::new(Expr::Identifier(ident)),
                                op: BinaryOp::Subtract,
                                right: Box::new(value),
                            },
                        })
                    }
                    TokenType::MultiplyAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(Stmt::Assign {
                            name: ident.clone(),
                            value: Expr::Binary {
                                left: Box::new(Expr::Identifier(ident)),
                                op: BinaryOp::Multiply,
                                right: Box::new(value),
                            },
                        })
                    }
                    TokenType::DivideAssign => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(Stmt::Assign {
                            name: ident.clone(),
                            value: Expr::Binary {
                                left: Box::new(Expr::Identifier(ident)),
                                op: BinaryOp::Divide,
                                right: Box::new(value),
                            },
                        })
                    }
                    _ => {
                        // Put the identifier back into an expression
                        self.position -= 1;
                        let expr = self.parse_expression()?;
                        Ok(Stmt::Expr(expr))
                    }
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_say(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'say'
        let expr = self.parse_expression()?;
        Ok(Stmt::Say(expr))
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
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

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'if'
        let condition = self.parse_expression()?;
        self.skip_newlines();

        let mut then_block = Vec::new();
        while !matches!(
            self.current_token().token_type,
            TokenType::Else | TokenType::Elif | TokenType::End | TokenType::Eof
        ) {
            then_block.push(self.parse_statement()?);
            self.skip_newlines();
        }

        let (else_block, is_chained) = if matches!(self.current_token().token_type, TokenType::Elif)
        {
            let else_if = self.parse_if()?;
            (Some(vec![else_if]), true)
        } else if matches!(self.current_token().token_type, TokenType::Else) {
            self.advance();

            // Check for 'else if'
            if matches!(self.current_token().token_type, TokenType::If) {
                let else_if = self.parse_if()?;
                (Some(vec![else_if]), true)
            } else {
                self.skip_newlines();
                let mut else_stmts = Vec::new();
                while !matches!(
                    self.current_token().token_type,
                    TokenType::End | TokenType::Eof
                ) {
                    else_stmts.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                (Some(else_stmts), false)
            }
        } else {
            (None, false)
        };

        if !is_chained {
            if !matches!(self.current_token().token_type, TokenType::End) {
                return Err(format!(
                    "Expected 'end' at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ));
            }
            self.advance();
        }

        Ok(Stmt::If {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'while'
        let condition = self.parse_expression()?;
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

        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'for'

        let var = match &self.current_token().token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => {
                return Err(format!(
                    "Expected identifier after 'for' at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ))
            }
        };
        self.advance();

        self.expect(TokenType::In)?;

        let start = self.parse_expression()?;
        self.expect(TokenType::DotDot)?;
        let end = self.parse_expression()?;
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

        Ok(Stmt::For {
            var,
            start,
            end,
            body,
        })
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
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

    fn parse_return(&mut self) -> Result<Stmt, String> {
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

    fn parse_when(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'when'
        let subject = self.parse_expression()?;
        self.skip_newlines();

        let mut arms = Vec::new();
        let mut else_block = None;

        while !matches!(
            self.current_token().token_type,
            TokenType::End | TokenType::Eof
        ) {
            if matches!(self.current_token().token_type, TokenType::Is) {
                self.advance(); // skip 'is'
                let pattern = self.parse_expression()?;
                self.expect(TokenType::Then)?;
                self.skip_newlines();
                let stmt = self.parse_statement()?;
                arms.push((pattern, stmt));
                self.skip_newlines();
            } else if matches!(self.current_token().token_type, TokenType::Else) {
                self.advance(); // skip 'else'
                self.skip_newlines();
                let mut else_stmts = Vec::new();
                while !matches!(
                    self.current_token().token_type,
                    TokenType::End | TokenType::Eof
                ) {
                    else_stmts.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                else_block = Some(else_stmts);
                break;
            } else {
                return Err(format!(
                    "Expected 'is' or 'else' in 'when' block at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ));
            }
        }

        self.expect(TokenType::End)?;

        // Desugar when into nested If statements
        let mut current_else = else_block;
        for (pattern, stmt) in arms.into_iter().rev() {
            let condition = Expr::Binary {
                left: Box::new(subject.clone()),
                op: BinaryOp::Equal,
                right: Box::new(pattern),
            };
            let if_stmt = Stmt::If {
                condition,
                then_block: vec![stmt],
                else_block: current_else,
            };
            current_else = Some(vec![if_stmt]);
        }

        match current_else {
            Some(mut stmts) if !stmts.is_empty() => Ok(stmts.remove(0)),
            _ => Err(format!(
                "Empty 'when' statement at line {}, column {}",
                self.current_token().line,
                self.current_token().column
            )),
        }
    }

    fn parse_try_catch(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'try'
        self.skip_newlines();

        let mut try_block = Vec::new();
        while !matches!(
            self.current_token().token_type,
            TokenType::Catch | TokenType::Eof
        ) {
            try_block.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(TokenType::Catch)?;

        // Optional catch variable: `catch err` or `catch`
        let catch_var = if let TokenType::Identifier(name) = &self.current_token().token_type {
            let name = name.clone();
            self.advance();
            Some(name)
        } else {
            None
        };
        self.skip_newlines();

        let mut catch_block = Vec::new();
        while !matches!(
            self.current_token().token_type,
            TokenType::End | TokenType::Eof
        ) {
            catch_block.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(TokenType::End)?;

        Ok(Stmt::TryCatch {
            try_block,
            catch_var,
            catch_block,
        })
    }
}
