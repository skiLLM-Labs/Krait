use crate::ast::{Expr, Literal, Op, Stmt};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &Token::Eof
        }
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn check(&self, tok: Token) -> bool {
        self.peek() == &tok
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let next = self.advance();
        if next == expected {
            Ok(())
        } else {
            Err(format!("Expected token {:?}, but found {:?}", expected, next))
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.advance() {
            Token::Ident(name) => Ok(name),
            other => Err(format!("Expected identifier, but found {:?}", other)),
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut program = Vec::new();
        while !self.check(Token::Eof) {
            if self.check(Token::Newline) {
                self.advance();
                continue;
            }
            program.push(self.parse_statement()?);
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Extern => {
                self.advance();
                self.expect(Token::Make)?;
                let name = self.expect_ident()?;
                self.expect(Token::LParen)?;
                let mut params = Vec::new();
                if !self.check(Token::RParen) {
                    loop {
                        params.push(self.expect_ident()?);
                        if self.check(Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RParen)?;
                self.expect(Token::Newline)?;
                Ok(Stmt::ExternDecl { name, params })
            }
            Token::Make => {
                self.advance();
                let name = self.expect_ident()?;
                if self.check(Token::LParen) {
                    self.advance();
                    let mut params = Vec::new();
                    if !self.check(Token::RParen) {
                        loop {
                            params.push(self.expect_ident()?);
                            if self.check(Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    let body = self.parse_block()?;
                    Ok(Stmt::FunctionDef { name, params, body })
                } else {
                    let body = self.parse_struct_body()?;
                    Ok(Stmt::StructDef { name, fields: body })
                }
            }
            Token::Set => {
                self.advance();
                let lhs = self.parse_expression()?;
                self.expect(Token::Equal)?;
                let value = self.parse_expression()?;
                self.expect(Token::Newline)?;
                match lhs {
                    Expr::Variable(name) => Ok(Stmt::VariableDecl { name, value }),
                    Expr::FieldAccess { object, field } => Ok(Stmt::FieldAssignment { object: *object, field, value }),
                    _ => Err("Invalid left-hand side in assignment".into()),
                }
            }
            Token::Import => {
                self.advance();
                match self.peek().clone() {
                    Token::Ident(name) => {
                        let n = name.clone();
                        self.advance();
                        Ok(Stmt::Import(n))
                    }
                    _ => Err("Expected module name after 'import'.".into())
                }
            }
            Token::When => {
                self.advance();
                let cond = self.parse_expression()?;
                let then_branch = self.parse_block()?;
                Ok(Stmt::When { cond, then_branch })
            }
            Token::Repeat => {
                self.advance();
                let count = self.parse_expression()?;
                self.expect(Token::Times)?;
                let body = self.parse_block()?;
                Ok(Stmt::Repeat { count, body })
            }
            Token::Show => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::Newline)?;
                Ok(Stmt::Show(expr))
            }
            Token::Return => {
                self.advance();
                let expr = if self.check(Token::Newline) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(Token::Newline)?;
                Ok(Stmt::Return(expr))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Newline)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let mut stmts = Vec::new();
        while !self.check(Token::Dedent) && !self.check(Token::Eof) {
            if self.check(Token::Newline) {
                self.advance();
                continue;
            }
            stmts.push(self.parse_statement()?);
        }
        self.expect(Token::Dedent)?;
        Ok(stmts)
    }

    fn parse_struct_body(&mut self) -> Result<Vec<(String, Expr)>, String> {
        self.expect(Token::Newline)?;
        self.expect(Token::Indent)?;
        let mut fields = Vec::new();
        while !self.check(Token::Dedent) && !self.check(Token::Eof) {
            if self.check(Token::Newline) {
                self.advance();
                continue;
            }
            let name = self.expect_ident()?;
            self.expect(Token::Equal)?;
            let value = self.parse_expression()?;
            self.expect(Token::Newline)?;
            fields.push((name, value));
        }
        self.expect(Token::Dedent)?;
        Ok(fields)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_relational()?;
        while self.check(Token::DoubleEqual) {
            self.advance();
            let right = self.parse_relational()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: Op::Eq,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_relational(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;
        while self.check(Token::Greater) || self.check(Token::Less) {
            let op = match self.advance() {
                Token::Greater => Op::Gt,
                Token::Less => Op::Lt,
                _ => unreachable!(),
            };
            let right = self.parse_additive()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;
        while self.check(Token::Plus) || self.check(Token::Minus) {
            let op = match self.advance() {
                Token::Plus => Op::Plus,
                Token::Minus => Op::Minus,
                _ => unreachable!(),
            };
            let right = self.parse_multiplicative()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        while self.check(Token::Star) || self.check(Token::Slash) {
            let op = match self.advance() {
                Token::Star => Op::Mul,
                Token::Slash => Op::Div,
                _ => unreachable!(),
            };
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let mut expr = match self.advance() {
            Token::Int(val) => Expr::Literal(Literal::Int(val)),
            Token::Float(val) => Expr::Literal(Literal::Float(val)),
            Token::Str(val) => Expr::Literal(Literal::Str(val)),
            Token::Bool(val) => Expr::Literal(Literal::Bool(val)),
            Token::New => {
                let name = self.expect_ident()?;
                Expr::New(name)
            }
            Token::Ident(name) => {
                if self.check(Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.check(Token::RParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.check(Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    Expr::Call { callee: name, args }
                } else {
                    Expr::Variable(name)
                }
            }
            Token::LParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                expr
            }
            other => return Err(format!("Expected expression, found invalid token: {:?}", other)),
        };

        while self.check(Token::Dot) {
            self.advance();
            let field = self.expect_ident()?;
            expr = Expr::FieldAccess {
                object: Box::new(expr),
                field,
            };
        }

        Ok(expr)
    }
}