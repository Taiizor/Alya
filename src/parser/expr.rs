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
                if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    let mut args = vec![expr];
                    if !matches!(self.current_token().token_type, TokenType::RightParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if matches!(self.current_token().token_type, TokenType::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenType::RightParen)?;
                    expr = Expr::Call { name: field, args };
                } else {
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field,
                    };
                }
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
                } else if ident == "map"
                    && matches!(self.current_token().token_type, TokenType::LeftBrace)
                {
                    self.parse_map_literal()
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
            TokenType::LeftBrace => self.parse_map_literal(),
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

    fn parse_map_literal(&mut self) -> Result<Expr, String> {
        self.expect(TokenType::LeftBrace)?;
        self.skip_newlines();
        let mut entries = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::RightBrace) {
            self.skip_newlines();
            if matches!(self.current_token().token_type, TokenType::RightBrace) {
                break;
            }
            let key = match &self.current_token().token_type {
                TokenType::Identifier(id) => {
                    let name = id.clone();
                    self.advance();
                    Expr::String(name)
                }
                _ => self.parse_expression()?,
            };
            self.skip_newlines();
            if matches!(
                self.current_token().token_type,
                TokenType::Colon | TokenType::Assign
            ) {
                self.advance();
            } else {
                return Err(format!(
                    "Expected ':' or '=' after map key at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ));
            }
            self.skip_newlines();
            let val = self.parse_expression()?;
            entries.push((key, val));
            self.skip_newlines();
            if matches!(self.current_token().token_type, TokenType::Comma) {
                self.advance();
                self.skip_newlines();
            } else if !matches!(self.current_token().token_type, TokenType::RightBrace) {
                return Err(format!(
                    "Expected ',' or '}}' after map value at line {}, column {}",
                    self.current_token().line,
                    self.current_token().column
                ));
            }
        }
        self.expect(TokenType::RightBrace)?;
        Ok(Expr::Map(entries))
    }
}

fn parse_interpolated_string(s: &str) -> Option<Vec<Expr>> {
    let mut parts = Vec::new();
    let mut current_lit = String::new();
    let mut chars = s.chars().peekable();
    let mut has_interpolation = false;
    let mut has_escaped = false;

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if chars.peek() == Some(&'{') {
                chars.next();
                current_lit.push('{');
                has_escaped = true;
                continue;
            }

            let mut expr_str = String::new();
            let mut depth = 1;
            let mut in_str = false;
            let mut in_escape = false;
            let mut found_close = false;

            for c in chars.by_ref() {
                if in_escape {
                    expr_str.push(c);
                    in_escape = false;
                } else if c == '\\' && in_str {
                    expr_str.push(c);
                    in_escape = true;
                } else if c == '"' {
                    in_str = !in_str;
                    expr_str.push(c);
                } else if !in_str && c == '{' {
                    depth += 1;
                    expr_str.push(c);
                } else if !in_str && c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        found_close = true;
                        break;
                    } else {
                        expr_str.push(c);
                    }
                } else {
                    expr_str.push(c);
                }
            }

            let mut parsed_expr = None;
            if found_close && !expr_str.trim().is_empty() {
                let mut sub_lexer = crate::lexer::Lexer::new(&expr_str);
                if let Ok(sub_tokens) = sub_lexer.tokenize() {
                    let mut sub_parser = Parser::new(sub_tokens);
                    if let Ok(expr) = sub_parser.parse_expression() {
                        if sub_parser.current_token().token_type == TokenType::Eof {
                            parsed_expr = Some(expr);
                        }
                    }
                }
            }

            if let Some(expr) = parsed_expr {
                has_interpolation = true;
                if !current_lit.is_empty() {
                    parts.push(Expr::String(current_lit.clone()));
                    current_lit.clear();
                }
                parts.push(expr);
            } else {
                current_lit.push('{');
                current_lit.push_str(&expr_str);
                if found_close {
                    current_lit.push('}');
                }
            }
        } else if ch == '}' && chars.peek() == Some(&'}') {
            chars.next();
            current_lit.push('}');
            has_escaped = true;
        } else {
            current_lit.push(ch);
        }
    }

    if !current_lit.is_empty() {
        parts.push(Expr::String(current_lit));
    }

    if has_interpolation || has_escaped {
        Some(parts)
    } else {
        None
    }
}
