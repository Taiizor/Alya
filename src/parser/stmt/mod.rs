mod control;
mod decl;

use crate::ast::*;
use crate::lexer::TokenType;
use crate::parser::Parser;

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

}
