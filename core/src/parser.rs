// core/src/parser.rs

use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    Custom(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Token,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer::new(src);
        let curr = lexer.next_token();
        Parser { lexer, curr }
    }

    fn advance(&mut self) {
        self.curr = self.lexer.next_token();
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<(), ParseError> {
        if &self.curr.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.curr.clone()))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while self.curr.kind != TokenKind::Eof {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match &self.curr.kind {
            TokenKind::Let => self.parse_let(),
            TokenKind::Fn => self.parse_function(),
            TokenKind::Return => self.parse_return(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(),
            TokenKind::Import => self.parse_import(),
            TokenKind::LBrace => self.parse_block_stmt(),
            _ => {
                let expr = self.parse_expr()?;
                if self.curr.kind == TokenKind::Semicolon {
                    self.advance();
                }
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Let)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(self.curr.clone()));
        };
        self.advance();
        self.expect(&TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Let { name, expr })
    }

    fn parse_function(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Fn)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(self.curr.clone()));
        };
        self.advance();
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        while self.curr.kind != TokenKind::RParen {
            if let TokenKind::Identifier(param) = &self.curr.kind {
                params.push(param.clone());
                self.advance();
                if self.curr.kind == TokenKind::Comma {
                    self.advance();
                } else if self.curr.kind != TokenKind::RParen {
                    return Err(ParseError::UnexpectedToken(self.curr.clone()));
                }
            } else {
                return Err(ParseError::UnexpectedToken(self.curr.clone()));
            }
        }
        self.expect(&TokenKind::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Return)?;
        let expr = if self.curr.kind != TokenKind::Semicolon {
            Some(self.parse_expr()?)
        } else { None };
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Return(expr))
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::If)?;
        let condition = self.parse_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.curr.kind == TokenKind::Else {
            self.advance();
            Some(self.parse_block()?)
        } else { None };
        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::While)?;
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::For)?;
        let var = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else { return Err(ParseError::UnexpectedToken(self.curr.clone())); };
        self.advance();
        self.expect(&TokenKind::In)?;
        let start = self.parse_expr()?;
        self.expect(&TokenKind::DotDot)?; // Not implemented in lexer, update logic if needed
        let end = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::For { var, start, end, body })
    }

    fn parse_import(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Import)?;
        let path = if let TokenKind::String(path) = &self.curr.kind {
            path.clone()
        } else { return Err(ParseError::UnexpectedToken(self.curr.clone())); };
        self.advance();
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Import(path))
    }

    fn parse_block_stmt(&mut self) -> Result<Stmt, ParseError> {
        Ok(Stmt::Block(self.parse_block()?))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(&TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while self.curr.kind != TokenKind::RBrace && self.curr.kind != TokenKind::Eof {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(stmts)
    }

    // --- Expression Parsing ---
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_logic_or()?;
        if self.curr.kind == TokenKind::Assign {
            self.advance();
            let value = self.parse_assignment()?;
            if let Expr::Variable(name) = expr {
                Ok(Expr::Assign { name, expr: Box::new(value) })
            } else {
                Err(ParseError::Custom("Invalid assignment target".to_string()))
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logic_and()?;
        while self.curr.kind == TokenKind::Or {
            self.advance();
            let right = self.parse_logic_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::Or,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn parse_logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;
        while self.curr.kind == TokenKind::And {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::And,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        while matches!(self.curr.kind, TokenKind::EqEq | TokenKind::BangEq) {
            let op = match &self.curr.kind {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::BangEq => BinOp::Neq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        while matches!(self.curr.kind, TokenKind::Lt | TokenKind::Lte | TokenKind::Gt | TokenKind::Gte) {
            let op = match &self.curr.kind {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Lte => BinOp::Lte,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Gte => BinOp::Gte,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        while matches!(self.curr.kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match &self.curr.kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        while matches!(self.curr.kind, TokenKind::Star | TokenKind::Slash) {
            let op = match &self.curr.kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match &self.curr.kind {
            TokenKind::Minus => {
                self.advance();
                Ok(Expr::Unary { op: UnOp::Neg, expr: Box::new(self.parse_unary()?) })
            }
            TokenKind::Bang => {
                self.advance();
                Ok(Expr::Unary { op: UnOp::Not, expr: Box::new(self.parse_unary()?) })
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.curr.kind == TokenKind::LParen {
                self.advance();
                let mut args = Vec::new();
                if self.curr.kind != TokenKind::RParen {
                    args.push(self.parse_expr()?);
                    while self.curr.kind == TokenKind::Comma {
                        self.advance();
                        args.push(self.parse_expr()?);
                    }
                }
                self.expect(&TokenKind::RParen)?;
                expr = Expr::Call { function: Box::new(expr), arguments: args };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match &self.curr.kind {
            TokenKind::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }
            TokenKind::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::String(val))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenKind::Identifier(name) => {
                let var = name.clone();
                self.advance();
                Ok(Expr::Variable(var))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBrace => {
                Ok(Expr::Block(self.parse_block()?))
            }
            _ => Err(ParseError::UnexpectedToken(self.curr.clone())),
        }
    }
}