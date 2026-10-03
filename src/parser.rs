#![allow(warnings)]
use crate::ast::*;
use crate::token::{Token, TokenType};

pub struct ParseError { pub line: usize, pub column: usize, pub message: String }

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn is_at_end(&self) -> bool { matches!(self.peek().token_type, TokenType::EOF) }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.pos += 1; }
        &self.tokens[self.pos - 1]
    }
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) { self.advance(); true } else { false }
    }
    fn consume(&mut self, token_type: &TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) { Ok(self.advance()) }
        else {
            let tok = self.peek();
            Err(ParseError { line: tok.line, column: tok.column, message: format!("{} (got {:?})", msg, tok.token_type) })
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut stmts = Vec::new();
        while !self.is_at_end() { stmts.push(self.statement()?); }
        Ok(Program { statements: stmts })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&TokenType::Let) { self.let_stmt() }
        else if self.match_token(&TokenType::Fn) { self.function_stmt() }
        else if self.match_token(&TokenType::Class) { self.class_stmt() }
        else if self.match_token(&TokenType::Return) { self.return_stmt() }
        else if self.match_token(&TokenType::Print) { self.print_stmt(false) }
        else if self.match_token(&TokenType::Println) { self.print_stmt(true) }
        else if self.match_token(&TokenType::If) { self.if_stmt() }
        else if self.match_token(&TokenType::While) { self.while_stmt() }
        else if self.match_token(&TokenType::For) { self.for_stmt() }
        else if self.match_token(&TokenType::Loop) { self.loop_stmt() }
        else if self.match_token(&TokenType::Break) {
            self.consume(&TokenType::Semicolon, "Expected ';' after break")?;
            Ok(Stmt::Break)
        } else if self.match_token(&TokenType::Step) { self.step_stmt() }
        else { self.expr_or_assign_stmt() }
    }

    fn function_stmt(&mut self) -> Result<Stmt, ParseError> {
        let func = self.function_decl()?;
        Ok(Stmt::Function(func))
    }

    fn function_decl(&mut self) -> Result<FunctionDecl, ParseError> {
        let name = match self.advance().token_type.clone() {
            TokenType::Ident(n) => n,
            _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected function name".into() }),
        };
        self.consume(&TokenType::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(&TokenType::RParen) {
            loop {
                if let TokenType::Ident(p) = self.advance().token_type.clone() {
                    params.push(p);
                }
                if !self.match_token(&TokenType::Comma) { break; }
            }
        }
        self.consume(&TokenType::RParen, "Expected ')' after parameters")?;
        self.consume(&TokenType::LBrace, "Expected '{' to start function body")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}' after function body")?;
        Ok(FunctionDecl { name, params, body })
    }

    fn class_stmt(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.advance().token_type.clone() {
            TokenType::Ident(n) => n,
            _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected class name".into() }),
        };
        self.consume(&TokenType::LBrace, "Expected '{' to start class body")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Fn) {
                methods.push(self.function_decl()?);
            } else if let TokenType::Ident(f) = self.advance().token_type.clone() {
                fields.push(f);
                self.consume(&TokenType::Semicolon, "Expected ';' after field declaration")?;
            }
        }
        self.consume(&TokenType::RBrace, "Expected '}' after class body")?;
        Ok(Stmt::Class(ClassDecl { name, fields, methods }))
    }

    fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = if !self.check(&TokenType::Semicolon) { Some(self.expression()?) } else { None };
        self.consume(&TokenType::Semicolon, "Expected ';' after return")?;
        Ok(Stmt::Return(expr))
    }

    fn let_stmt(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.advance().token_type.clone() {
            TokenType::Ident(n) => n,
            _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected variable name after let".into() }),
        };
        self.consume(&TokenType::Assign, "Expected '=' after variable name")?;
        let init = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after let declaration")?;
        Ok(Stmt::Let(name, init))
    }

    fn print_stmt(&mut self, is_newline: bool) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after print")?;
        let expr = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after argument")?;
        self.consume(&TokenType::Semicolon, "Expected ';' after print call")?;
        if is_newline { Ok(Stmt::Println(expr)) } else { Ok(Stmt::Print(expr)) }
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after if")?;
        let cond = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after if condition")?;
        self.consume(&TokenType::LBrace, "Expected '{'")?;
        let mut then_branch = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { then_branch.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}'")?;
        let else_branch = if self.match_token(&TokenType::Else) {
            if self.match_token(&TokenType::If) { Some(vec![self.if_stmt()?]) }
            else {
                self.consume(&TokenType::LBrace, "Expected '{'")?;
                let mut el = Vec::new();
                while !self.check(&TokenType::RBrace) && !self.is_at_end() { el.push(self.statement()?); }
                self.consume(&TokenType::RBrace, "Expected '}'")?;
                Some(el)
            }
        } else { None };
        Ok(Stmt::If { cond, then_branch, else_branch })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after while")?;
        let cond = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')'")?;
        self.consume(&TokenType::LBrace, "Expected '{'")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}'")?;
        Ok(Stmt::While { cond, body })
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        let var = match self.advance().token_type.clone() {
            TokenType::Ident(n) => n,
            _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected loop variable name".into() }),
        };
        self.consume(&TokenType::In, "Expected 'in' in for loop")?;
        let iter = self.expression()?;
        self.consume(&TokenType::LBrace, "Expected '{'")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}'")?;
        Ok(Stmt::For { var, iter, body })
    }

    fn loop_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LBrace, "Expected '{'")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}'")?;
        Ok(Stmt::Loop { body })
    }

    fn step_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after step")?;
        let dt_var = match self.advance().token_type.clone() {
            TokenType::Ident(n) => n,
            _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected dt identifier".into() }),
        };
        self.consume(&TokenType::RParen, "Expected ')'")?;
        self.consume(&TokenType::LBrace, "Expected '{'")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}'")?;
        Ok(Stmt::Step { dt_var, body })
    }

    fn expr_or_assign_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        if self.match_token(&TokenType::Assign) {
            let val = self.expression()?;
            self.consume(&TokenType::Semicolon, "Expected ';' after assignment")?;
            match expr {
                Expr::Ident(name) => Ok(Stmt::Assign(name, val)),
                Expr::MemberAccess(obj, field) => Ok(Stmt::MemberAssign(obj, field, val)),
                Expr::IndexAccess(arr, idx) => Ok(Stmt::IndexAssign(arr, idx, val)),
                _ => Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Invalid assignment target".into() }),
            }
        } else {
            self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
            Ok(Stmt::Expr(expr))
        }
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> { self.logical_or() }

    fn logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logical_and()?;
        while self.match_token(&TokenType::OrOr) {
            let right = self.logical_and()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::Or, Box::new(right));
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_token(&TokenType::AndAnd) {
            let right = self.equality()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::And, Box::new(right));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while let Some(op) = match self.peek().token_type {
            TokenType::EqualEqual => Some(BinaryOp::Eq),
            TokenType::BangEqual => Some(BinaryOp::Neq),
            _ => None,
        } {
            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while let Some(op) = match self.peek().token_type {
            TokenType::Less => Some(BinaryOp::Lt),
            TokenType::LessEqual => Some(BinaryOp::Lte),
            TokenType::Greater => Some(BinaryOp::Gt),
            TokenType::GreaterEqual => Some(BinaryOp::Gte),
            _ => None,
        } {
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while let Some(op) = match self.peek().token_type {
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Sub),
            _ => None,
        } {
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while let Some(op) = match self.peek().token_type {
            TokenType::Star => Some(BinaryOp::Mul),
            TokenType::Slash => Some(BinaryOp::Div),
            TokenType::Percent => Some(BinaryOp::Mod),
            _ => None,
        } {
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&TokenType::Minus) {
            let expr = self.unary()?;
            Ok(Expr::Unary(UnaryOp::Neg, Box::new(expr)))
        } else if self.match_token(&TokenType::Bang) {
            let expr = self.unary()?;
            Ok(Expr::Unary(UnaryOp::Not, Box::new(expr)))
        } else {
            self.call_or_member()
        }
    }

    fn call_or_member(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::DotOp) {
                let member = match self.advance().token_type.clone() {
                    TokenType::Ident(n) => n,
                    _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected member identifier after '.'".into() }),
                };

                if self.match_token(&TokenType::LParen) {
                    let mut args = Vec::new();
                    if !self.check(&TokenType::RParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) { break; }
                        }
                    }
                    self.consume(&TokenType::RParen, "Expected ')' after method arguments")?;
                    expr = Expr::MethodCall(Box::new(expr), member, args);
                } else {
                    expr = Expr::MemberAccess(Box::new(expr), member);
                }
            } else if self.match_token(&TokenType::LBracket) {
                let index = self.expression()?;
                self.consume(&TokenType::RBracket, "Expected ']' after array index")?;
                expr = Expr::IndexAccess(Box::new(expr), Box::new(index));
            } else if self.match_token(&TokenType::LParen) {
                if let Expr::Ident(name) = expr {
                    let mut args = Vec::new();
                    if !self.check(&TokenType::RParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_token(&TokenType::Comma) { break; }
                        }
                    }
                    self.consume(&TokenType::RParen, "Expected ')' after arguments")?;
                    expr = Expr::Call(name, args);
                } else {
                    return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Invalid function call target".into() });
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let tok = self.advance().clone();
        match tok.token_type {
            TokenType::Int(n) => Ok(Expr::Int(n)),
            TokenType::Float(f) => Ok(Expr::Float(f)),
            TokenType::Str(s) => Ok(Expr::Str(s)),
            TokenType::True => Ok(Expr::Bool(true)),
            TokenType::False => Ok(Expr::Bool(false)),
            TokenType::Nil => Ok(Expr::Nil),
            TokenType::This => Ok(Expr::This),
            TokenType::Ident(name) => Ok(Expr::Ident(name)),
            TokenType::LParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(expr)
            }
            TokenType::LBracket => {
                let mut elements = Vec::new();
                if !self.check(&TokenType::RBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&TokenType::Comma) { break; }
                    }
                }
                self.consume(&TokenType::RBracket, "Expected ']' after list")?;
                Ok(Expr::Array(elements))
            }
            TokenType::New => {
                let class_name = match self.advance().token_type.clone() {
                    TokenType::Ident(n) => n,
                    _ => return Err(ParseError { line: self.peek().line, column: self.peek().column, message: "Expected class name after 'new'".into() }),
                };
                self.consume(&TokenType::LParen, "Expected '(' after class name")?;
                let mut args = Vec::new();
                if !self.check(&TokenType::RParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.match_token(&TokenType::Comma) { break; }
                    }
                }
                self.consume(&TokenType::RParen, "Expected ')' after constructor arguments")?;
                Ok(Expr::New(class_name, args))
            }
            TokenType::Vec2 => {
                self.consume(&TokenType::LParen, "Expected '(' after vec2")?;
                let x = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let y = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Vec2(Box::new(x), Box::new(y)))
            }
            TokenType::Vec3 => {
                self.consume(&TokenType::LParen, "Expected '(' after vec3")?;
                let x = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let y = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let z = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Vec3(Box::new(x), Box::new(y), Box::new(z)))
            }
            TokenType::Vec4 => {
                self.consume(&TokenType::LParen, "Expected '(' after vec4")?;
                let x = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let y = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let z = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let w = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Vec4(Box::new(x), Box::new(y), Box::new(z), Box::new(w)))
            }
            TokenType::Dot => {
                self.consume(&TokenType::LParen, "Expected '(' after dot")?;
                let a = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let b = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Dot(Box::new(a), Box::new(b)))
            }
            TokenType::Cross => {
                self.consume(&TokenType::LParen, "Expected '(' after cross")?;
                let a = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let b = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Cross(Box::new(a), Box::new(b)))
            }
            TokenType::Length => {
                self.consume(&TokenType::LParen, "Expected '(' after length")?;
                let v = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Length(Box::new(v)))
            }
            TokenType::Normalize => {
                self.consume(&TokenType::LParen, "Expected '(' after normalize")?;
                let v = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::Normalize(Box::new(v)))
            }
            TokenType::Input => {
                self.consume(&TokenType::LParen, "Expected '(' after input")?;
                let prompt = if !self.check(&TokenType::RParen) { Some(Box::new(self.expression()?)) } else { None };
                self.consume(&TokenType::RParen, "Expected ')' after input")?;
                Ok(Expr::Input(prompt))
            }
            TokenType::ReadFile => {
                self.consume(&TokenType::LParen, "Expected '(' after read_file")?;
                let path = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::ReadFile(Box::new(path)))
            }
            TokenType::WriteFile => {
                self.consume(&TokenType::LParen, "Expected '(' after write_file")?;
                let path = self.expression()?; self.consume(&TokenType::Comma, "Expected ','")?;
                let content = self.expression()?; self.consume(&TokenType::RParen, "Expected ')'")?;
                Ok(Expr::WriteFile(Box::new(path), Box::new(content)))
            }
            _ => Err(ParseError { line: tok.line, column: tok.column, message: format!("Unexpected expression token: {:?}", tok.token_type) }),
        }
    }
}
