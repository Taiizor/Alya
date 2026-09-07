use super::Parser;
use crate::ast::*;
use crate::lexer::TokenType;

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();

        match &self.current_token().token_type {
            TokenType::Import => self.parse_import(),
            TokenType::Struct => self.parse_struct(),
            TokenType::Say => self.parse_say(),
            TokenType::Let => self.parse_let(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::Repeat => self.parse_repeat(),
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

                if matches!(
                    self.current_token().token_type,
                    TokenType::LeftBracket | TokenType::Dot
                ) {
                    let mut target = Expr::Identifier(ident);
                    while matches!(
                        self.current_token().token_type,
                        TokenType::LeftBracket | TokenType::Dot
                    ) {
                        if matches!(self.current_token().token_type, TokenType::LeftBracket) {
                            self.advance();
                            let index = self.parse_expression()?;
                            self.expect(TokenType::RightBracket)?;
                            target = Expr::Index {
                                array: Box::new(target),
                                index: Box::new(index),
                            };
                        } else if matches!(self.current_token().token_type, TokenType::Dot) {
                            self.advance();
                            let field = match &self.current_token().token_type {
                                TokenType::Identifier(s) => s.clone(),
                                _ => {
                                    return Err(format!(
                                        "Expected field name after '.' at line {}, column {}",
                                        self.current_token().line,
                                        self.current_token().column
                                    ))
                                }
                            };
                            self.advance();
                            if matches!(self.current_token().token_type, TokenType::LeftParen) {
                                self.advance();
                                let mut args = vec![target];
                                if !matches!(self.current_token().token_type, TokenType::RightParen)
                                {
                                    loop {
                                        args.push(self.parse_expression()?);
                                        if matches!(
                                            self.current_token().token_type,
                                            TokenType::Comma
                                        ) {
                                            self.advance();
                                        } else {
                                            break;
                                        }
                                    }
                                }
                                self.expect(TokenType::RightParen)?;
                                target = Expr::Call { name: field, args };
                            } else {
                                target = Expr::FieldAccess {
                                    object: Box::new(target),
                                    field,
                                };
                            }
                        }
                    }

                    match self.current_token().token_type {
                        TokenType::Assign => {
                            self.advance();
                            let value = self.parse_expression()?;
                            match target {
                                Expr::Index { array, index } => {
                                    return Ok(Stmt::IndexAssign {
                                        array: *array,
                                        index: *index,
                                        value,
                                    });
                                }
                                Expr::FieldAccess { object, field } => {
                                    return Ok(Stmt::FieldAssign {
                                        object: *object,
                                        field,
                                        value,
                                    });
                                }
                                _ => unreachable!(),
                            }
                        }
                        TokenType::PlusAssign
                        | TokenType::MinusAssign
                        | TokenType::MultiplyAssign
                        | TokenType::DivideAssign => {
                            let bin_op = match self.current_token().token_type {
                                TokenType::PlusAssign => BinaryOp::Add,
                                TokenType::MinusAssign => BinaryOp::Subtract,
                                TokenType::MultiplyAssign => BinaryOp::Multiply,
                                TokenType::DivideAssign => BinaryOp::Divide,
                                _ => unreachable!(),
                            };
                            self.advance();
                            let value = self.parse_expression()?;
                            let bin_val = Expr::Binary {
                                left: Box::new(target.clone()),
                                op: bin_op,
                                right: Box::new(value),
                            };
                            match target {
                                Expr::Index { array, index } => {
                                    return Ok(Stmt::IndexAssign {
                                        array: *array,
                                        index: *index,
                                        value: bin_val,
                                    });
                                }
                                Expr::FieldAccess { object, field } => {
                                    return Ok(Stmt::FieldAssign {
                                        object: *object,
                                        field,
                                        value: bin_val,
                                    });
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => return Ok(Stmt::Expr(target)),
                    }
                }

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

    fn parse_import(&mut self) -> Result<Stmt, String> {
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

    fn parse_repeat(&mut self) -> Result<Stmt, String> {
        self.advance(); // skip 'repeat'
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

        Ok(Stmt::Repeat { body })
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

        let expr = self.parse_expression()?;
        if matches!(self.current_token().token_type, TokenType::DotDot) {
            self.advance();
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
                start: expr,
                end,
                body,
            })
        } else {
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

            Ok(Stmt::ForEach {
                var,
                iterable: expr,
                body,
            })
        }
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

    fn parse_struct(&mut self) -> Result<Stmt, String> {
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
