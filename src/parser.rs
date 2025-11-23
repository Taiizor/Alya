use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current_token(&self) -> &Token {
        if self.position < self.tokens.len() {
            &self.tokens[self.position]
        } else {
            self.tokens.last().unwrap()
        }
    }

    fn peek_token(&self) -> &Token {
        if self.position + 1 < self.tokens.len() {
            &self.tokens[self.position + 1]
        } else {
            self.tokens.last().unwrap()
        }
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current_token().token_type, TokenType::Newline) {
            self.advance();
        }
    }

    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.current_token().token_type) != std::mem::discriminant(&expected) {
            return Err(format!(
                "Expected {:?}, found {:?} at line {}, column {}",
                expected,
                self.current_token().token_type,
                self.current_token().line,
                self.current_token().column
            ));
        }
        self.advance();
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        self.skip_newlines();

        while !matches!(self.current_token().token_type, TokenType::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
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
            TokenType::Identifier(_) => {
                // Could be assignment or function call
                let ident = match &self.current_token().token_type {
                    TokenType::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };
                self.advance();

                if matches!(self.current_token().token_type, TokenType::Assign) {
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assign { name: ident, value })
                } else {
                    // Put the identifier back into an expression
                    self.position -= 1;
                    let expr = self.parse_expression()?;
                    Ok(Stmt::Expr(expr))
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
            _ => return Err(format!("Expected identifier after 'let' at line {}", self.current_token().line)),
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
            TokenType::Else | TokenType::End | TokenType::Eof
        ) {
            then_block.push(self.parse_statement()?);
            self.skip_newlines();
        }

        let else_block = if matches!(self.current_token().token_type, TokenType::Else) {
            self.advance();
            
            // Check for 'else if'
            if matches!(self.current_token().token_type, TokenType::If) {
                let else_if = self.parse_if()?;
                Some(vec![else_if])
            } else {
                self.skip_newlines();
                let mut else_stmts = Vec::new();
                while !matches!(self.current_token().token_type, TokenType::End | TokenType::Eof) {
                    else_stmts.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                Some(else_stmts)
            }
        } else {
            None
        };

        if !matches!(self.current_token().token_type, TokenType::End) {
            return Err(format!("Expected 'end' at line {}", self.current_token().line));
        }
        self.advance();

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
        while !matches!(self.current_token().token_type, TokenType::End | TokenType::Eof) {
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
            _ => return Err(format!("Expected identifier after 'for' at line {}", self.current_token().line)),
        };
        self.advance();

        self.expect(TokenType::In)?;

        let start = self.parse_expression()?;
        self.expect(TokenType::DotDot)?;
        let end = self.parse_expression()?;
        self.skip_newlines();

        let mut body = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::End | TokenType::Eof) {
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
            _ => return Err(format!("Expected function name at line {}", self.current_token().line)),
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
                return Err(format!("Expected parameter name at line {}", self.current_token().line));
            }
        }

        self.expect(TokenType::RightParen)?;
        self.skip_newlines();

        let mut body = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::End | TokenType::Eof) {
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

    fn parse_expression(&mut self) -> Result<Expr, String> {
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
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token().token_type {
            TokenType::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            TokenType::String(s) => {
                let string = s.clone();
                self.advance();
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
            _ => Err(format!(
                "Unexpected token {:?} at line {}, column {}",
                self.current_token().token_type,
                self.current_token().line,
                self.current_token().column
            )),
        }
    }
}
