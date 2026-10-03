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
    fn match_token(&mut self, token_type: &TokenType) ->Here is **Step 5 (`parser.rs`)**. To ensure Termux does not truncate the text, this creates the file in two safe parts.

Copy and paste **Part A**:

```bash
cat << 'EOF' > /sdcard/GAGE/src/parser.rs
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
        else if self.match_token(&TokenType::Print) { self.print_stmt(false) }
        else if self.match_token(&TokenType::Println) { self.print_stmt(true) }
        else if self.match_token(&TokenType::If) { self.if_stmt() }
        else if self.match_token(&TokenType::While) { self.while_stmt() }
        else if self.match_token(&TokenType::Loop) { self.loop_stmt() }
        else if self.match_token(&TokenType::Break) {
            self.consume(&TokenType::Semicolon, "Expected ';' after break")?;
            Ok(Stmt::Break)
        } else if self.match_token(&TokenType::Step) { self.step_stmt() }
        else { self.expr_or_assign_stmt() }
    }

    fn let_stmt(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.advance().token_type.clone() {
            TokenType::Ident(n) => n,
            _ => {
                let t = self.peek();
                return Err(ParseError { line: t.line, column: t.column, message: "Expected variable name after let".into() });
            }
        };
        self.consume(&TokenType::Assign, "Expected '=' after variable name")?;
        let init = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after variable declaration")?;
        Ok(Stmt::Let(name, init))
    }

    fn print_stmt(&mut self, is_newline: bool) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after print/println")?;
        let expr = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after expression")?;
        self.consume(&TokenType::Semicolon, "Expected ';' after print statement")?;
        if is_newline { Ok(Stmt::Println(expr)) } else { Ok(Stmt::Print(expr)) }
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after 'if'")?;
        let cond = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after condition")?;
        self.consume(&TokenType::LBrace, "Expected '{' to start if block")?;
        let mut then_branch = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { then_branch.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}' after then block")?;

        let else_branch = if self.match_token(&TokenType::Else) {
            if self.match_token(&TokenType::If) {
                Some(vec![self.if_stmt()?])
            } else {
                self.consume(&TokenType::LBrace, "Expected '{' to start else block")?;
                let mut el = Vec::new();
                while !self.check(&TokenType::RBrace) && !self.is_at_end() { el.push(self.statement()?); }
                self.consume(&TokenType::RBrace, "Expected '}' after else block")?;
                Some(el)
            }
        } else { None };
        Ok(Stmt::If { cond, then_branch, else_branch })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after 'while'")?;
        let cond = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after while condition")?;
        self.consume(&TokenType::LBrace, "Expected '{' to start while block")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}' after while block")?;
        Ok(Stmt::While { cond, body })
    }

    fn loop_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LBrace, "Expected '{' to start loop block")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}' after loop block")?;
        Ok(Stmt::Loop { body })
    }

    fn step_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after 'step'")?;
        let dt_var = match self.advance().token_type.clone() {
            TokenType::Ident(name) => name,
            _ => {
                let t = self.peek();
                return Err(ParseError { line: t.line, column: t.column, message: "Expected dt identifier inside step(...)".into() });
            }
        };
        self.consume(&TokenType::RParen, "Expected ')' after dt identifier")?;
        self.consume(&TokenType::LBrace, "Expected '{' to start step block")?;
        let mut body = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() { body.push(self.statement()?); }
        self.consume(&TokenType::RBrace, "Expected '}' after step block")?;
        Ok(Stmt::Step { dt_var, body })
    }

    fn expr_or_assign_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start_pos = self.pos;
        if let TokenType::Ident(name) = self.peek().token_type.clone() {
            self.advance();
            if self.match_token(&TokenType::Assign) {
                let val = self.expression()?;
                self.consume(&TokenType::Semicolon, "Expected ';' after assignment")?;
                return Ok(Stmt::Assign(name, val));
            }
            self.pos = start_pos;
        }
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expr(expr))
    }
