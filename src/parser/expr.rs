use super::Parser;
use crate::ast::*;
use crate::lexer::TokenType;

impl Parser {
    pub(super) fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;

        while matches!(self.current_token().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current_token().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while let Some(op) = match &self.current_token().token_type {
            TokenType::Equal => Some(BinaryOp::Equal),
            TokenType::NotEqual => Some(BinaryOp::NotEqual),
            TokenType::Less => Some(BinaryOp::Less),
            TokenType::Greater => Some(BinaryOp::Greater),
            TokenType::LessEqual => Some(BinaryOp::LessEqual),
            TokenType::GreaterEqual => Some(BinaryOp::GreaterEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while let Some(op) = match &self.current_token().token_type {
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Subtract),
            _ => None,
        } {
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        while let Some(op) = match &self.current_token().token_type {
            TokenType::Multiply => Some(BinaryOp::Multiply),
            TokenType::Divide => Some(BinaryOp::Divide),
            TokenType::Modulo => Some(BinaryOp::Modulo),
            _ => None,
        } {
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match &self.current_token().token_type {
            TokenType::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(expr),
                })
            }
            TokenType::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if matches!(self.current_token().token_type, TokenType::LeftBracket) {
                self.advance();
                let index = self.parse_expression()?;
                self.expect(TokenType::RightBracket)?;
                expr = Expr::Index {
                    array: Box::new(expr),
                    index: Box::new(index),
                };
            } else if matches!(self.current_token().token_type, TokenType::Dot) {
                self.advance();
                let field = match &self.current_token().token_type {
                    TokenType::Identifier(f) => f.clone(),
                    _ => {
                        return Err(format!(
                            "Expected field name after '.' at line {}, column {}",
                            self.current_token().line,
                            self.current_token().column
                        ))
                    }
                };
                self.advance();
                expr = Expr::FieldAccess {
                    object: Box::new(expr),
                    field,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token().token_type {
            TokenType::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            TokenType::Float(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Float(num))
            }
            TokenType::String(s) => {
                let string = s.clone();
                self.advance();
                if string.contains('{') && string.contains('}') {
                    if let Some(parts) = parse_interpolated_string(&string) {
                        return Ok(Expr::InterpolatedString(parts));
                    }
                }
                Ok(Expr::String(string))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Number(1.0))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Number(0.0))
            }
            TokenType::Ask => {
                self.advance();
                let mut args = Vec::new();
                if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    if !matches!(self.current_token().token_type, TokenType::RightParen) {
                        args.push(self.parse_expression()?);
                    }
                    self.expect(TokenType::RightParen)?;
                } else if matches!(self.current_token().token_type, TokenType::String(_)) {
                    args.push(self.parse_primary()?);
                }
                Ok(Expr::Call {
                    name: "ask".into(),
                    args,
                })
            }
            TokenType::Identifier(name) => {
                let ident = name.clone();
                self.advance();

                // Check for function call
                if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();

                    while !matches!(self.current_token().token_type, TokenType::RightParen) {
                        args.push(self.parse_expression()?);
                        if matches!(self.current_token().token_type, TokenType::Comma) {
                            self.advance();
                        }
                    }

                    self.expect(TokenType::RightParen)?;

                    Ok(Expr::Call { name: ident, args })
                } else if matches!(self.current_token().token_type, TokenType::LeftBrace) {
                    self.advance();
                    self.skip_newlines();
                    let mut fields = Vec::new();

                    while !matches!(self.current_token().token_type, TokenType::RightBrace) {
                        self.skip_newlines();
                        if matches!(self.current_token().token_type, TokenType::RightBrace) {
                            break;
                        }
                        let field_name = match &self.current_token().token_type {
                            TokenType::Identifier(f) => f.clone(),
                            _ => {
                                return Err(format!(
                                    "Expected field name in struct initialization at line {}, column {}",
                                    self.current_token().line,
                                    self.current_token().column
                                ))
                            }
                        };
                        self.advance();
                        if matches!(
                            self.current_token().token_type,
                            TokenType::Colon | TokenType::Assign
                        ) {
                            self.advance();
                        } else {
                            return Err(format!(
                                "Expected ':' or '=' after field name at line {}, column {}",
                                self.current_token().line,
                                self.current_token().column
                            ));
                        }
                        let val = self.parse_expression()?;
                        fields.push((field_name, val));
                        self.skip_newlines();
                        if matches!(self.current_token().token_type, TokenType::Comma) {
                            self.advance();
                            self.skip_newlines();
                        }
                    }

                    self.expect(TokenType::RightBrace)?;
                    Ok(Expr::StructInit {
                        name: ident,
                        fields,
                    })
                } else {
                    Ok(Expr::Identifier(ident))
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                self.skip_newlines();
                let mut elements = Vec::new();
                while !matches!(self.current_token().token_type, TokenType::RightBracket) {
                    elements.push(self.parse_expression()?);
                    self.skip_newlines();
                    if matches!(self.current_token().token_type, TokenType::Comma) {
                        self.advance();
                        self.skip_newlines();
                    } else if !matches!(self.current_token().token_type, TokenType::RightBracket) {
                        return Err(format!(
                            "Expected ',' or ']' after array element at line {}, column {}",
                            self.current_token().line,
                            self.current_token().column
                        ));
                    }
                }
                self.expect(TokenType::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            _ => Err(format!(
                "Unexpected token {} at line {}, column {}",
                self.current_token().token_type,
                self.current_token().line,
                self.current_token().column
            )),
        }
    }
}

fn parse_interpolated_string(s: &str) -> Option<Vec<Expr>> {
    let mut parts = Vec::new();
    let mut current_lit = String::new();
    let mut chars = s.chars().peekable();
    let mut has_interpolation = false;

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut var_name = String::new();
            let mut found_close = false;
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    chars.next();
                    found_close = true;
                    break;
                } else if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '.' {
                    var_name.push(next_ch);
                    chars.next();
                } else {
                    break;
                }
            }

            if found_close && !var_name.is_empty() {
                has_interpolation = true;
                if !current_lit.is_empty() {
                    parts.push(Expr::String(current_lit.clone()));
                    current_lit.clear();
                }
                if var_name.contains('.') {
                    let subparts: Vec<&str> = var_name.split('.').collect();
                    let mut expr = Expr::Identifier(subparts[0].to_string());
                    for &field in &subparts[1..] {
                        expr = Expr::FieldAccess {
                            object: Box::new(expr),
                            field: field.to_string(),
                        };
                    }
                    parts.push(expr);
                } else {
                    parts.push(Expr::Identifier(var_name));
                }
            } else {
                current_lit.push('{');
                current_lit.push_str(&var_name);
            }
        } else {
            current_lit.push(ch);
        }
    }

    if !current_lit.is_empty() {
        parts.push(Expr::String(current_lit));
    }

    if has_interpolation {
        Some(parts)
    } else {
        None
    }
}
