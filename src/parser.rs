use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            // Skip redundant semicolons between statements
            if self.match_token(&TokenType::Semicolon) {
                continue;
            }
            statements.push(self.declaration()?);
        }
        Ok(Program { statements })
    }

    // --- Declarations ---
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&TokenType::Function) {
            return self.function_declaration();
        }
        if self.match_token(&TokenType::Let) {
            return self.let_declaration();
        }
        self.statement()
    }

    fn function_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.advance().token_type {
            TokenType::Ident(n) => n,
            _ => return Err(self.error("Expected function name")),
        };

        self.consume(&TokenType::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();

        if !self.check(&TokenType::RParen) {
            loop {
                let param_name = match self.advance().token_type {
                    TokenType::Ident(n) => n,
                    _ => return Err(self.error("Expected parameter name")),
                };

                let mut type_ann = None;
                if self.match_token(&TokenType::Colon) {
                    type_ann = Some(self.parse_type_name()?);
                }
                params.push((param_name, type_ann));

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RParen, "Expected ')' after parameters")?;

        let mut return_type = None;
        if self.match_token(&TokenType::FatArrow) {
            // Check if arrow points to return type or single-expression body
            if self.is_type_token() {
                return_type = Some(self.parse_type_name()?);
            }
        }

        // Body can be a block '{ ... }' or single-expression shorthand
        let body = if self.check(&TokenType::LBrace) {
            self.block_statement()?
        } else {
            let expr = self.expression()?;
            self.match_token(&TokenType::Semicolon);
            Stmt::Return(Some(expr))
        };

        Ok(Stmt::Function {
            name,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    fn let_declaration(&mut self) -> Result<Stmt, ParseError> {
        let is_mut = self.match_token(&TokenType::Mut);

        let name = match self.advance().token_type {
            TokenType::Ident(n) => n,
            _ => return Err(self.error("Expected variable name after 'let'")),
        };

        let mut type_annotation = None;
        if self.match_token(&TokenType::Colon) {
            type_annotation = Some(self.parse_type_name()?);
        }

        self.consume(&TokenType::Equal, "Expected '=' in variable declaration")?;
        let initializer = self.expression()?;
        self.match_token(&TokenType::Semicolon);

        Ok(Stmt::Let {
            name,
            is_mut,
            type_annotation,
            initializer,
        })
    }

    // --- Statements ---
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&TokenType::If) {
            return self.if_statement();
        }
        if self.match_token(&TokenType::While) {
            return self.while_statement();
        }
        if self.match_token(&TokenType::Loop) {
            return self.loop_statement();
        }
        if self.match_token(&TokenType::Step) {
            return self.step_statement();
        }
        if self.match_token(&TokenType::Break) {
            self.match_token(&TokenType::Semicolon);
            return Ok(Stmt::Break);
        }
        if self.match_token(&TokenType::Continue) {
            self.match_token(&TokenType::Semicolon);
            return Ok(Stmt::Continue);
        }
        if self.match_token(&TokenType::Return) {
            return self.return_statement();
        }
        if self.check(&TokenType::LBrace) {
            return self.block_statement();
        }

        self.expression_or_assignment()
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after condition")?;

        let then_branch = Box::new(self.block_statement()?);
        let mut else_branch = None;

        if self.match_token(&TokenType::Else) {
            if self.match_token(&TokenType::If) {
                else_branch = Some(Box::new(self.if_statement()?));
            } else {
                else_branch = Some(Box::new(self.block_statement()?));
            }
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after while condition")?;
        let body = Box::new(self.block_statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn loop_statement(&mut self) -> Result<Stmt, ParseError> {
        let body = Box::new(self.block_statement()?);
        Ok(Stmt::Loop { body })
    }

    fn step_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LParen, "Expected '(' after 'step'")?;
        let dt_ident = match self.advance().token_type {
            TokenType::Ident(id) => id,
            _ => return Err(self.error("Expected delta-time variable identifier in 'step(dt)'")),
        };
        self.consume(&TokenType::RParen, "Expected ')' after step variable")?;
        let body = Box::new(self.block_statement()?);
        Ok(Stmt::Step { dt_ident, body })
    }

    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LBrace, "Expected '{'")?;
        let mut stmts = Vec::new();

        while !self.check(&TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Semicolon) {
                continue;
            }
            stmts.push(self.declaration()?);
        }

        self.consume(&TokenType::RBrace, "Expected '}' after block")?;
        Ok(Stmt::Block(stmts))
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut value = None;
        if !self.check(&TokenType::Semicolon) && !self.check(&TokenType::RBrace) && !self.is_at_end() {
            value = Some(self.expression()?);
        }
        self.match_token(&TokenType::Semicolon);
        Ok(Stmt::Return(value))
    }

    fn expression_or_assignment(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        // If expression is followed by '=', it is an assignment
        if self.match_token(&TokenType::Equal) {
            if let Expr::Variable(name) = expr {
                let value = self.expression()?;
                self.match_token(&TokenType::Semicolon);
                return Ok(Stmt::Assign { name, value });
            } else {
                return Err(self.error("Invalid assignment target"));
            }
        }

        self.match_token(&TokenType::Semicolon);
        Ok(Stmt::Expression(expr))
    }

    // --- Expressions (Precedence Climbing) ---
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while let Some(op) = self.match_equality_op() {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while let Some(op) = self.match_comparison_op() {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while let Some(op) = self.match_term_op() {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while let Some(op) = self.match_factor_op() {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&TokenType::Minus) {
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(expr),
            });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let expr = self.primary()?;

        if let Expr::Variable(name) = &expr {
            // Function or Vector call: name(...)
            if self.match_token(&TokenType::LParen) {
                let mut args = Vec::new();
                if !self.check(&TokenType::RParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenType::RParen, "Expected ')' after arguments")?;

                // Built-in vector constructors
                return match name.as_str() {
                    "vec2" => {
                        if args.len() != 2 {
                            return Err(self.error("vec2 requires exactly 2 arguments"));
                        }
                        Ok(Expr::Vec2(Box::new(args.remove(0)), Box::new(args.remove(0))))
                    }
                    "vec3" => {
                        if args.len() != 3 {
                            return Err(self.error("vec3 requires exactly 3 arguments"));
                        }
                        Ok(Expr::Vec3(
                            Box::new(args.remove(0)),
                            Box::new(args.remove(0)),
                            Box::new(args.remove(0)),
                        ))
                    }
                    "vec4" => {
                        if args.len() != 4 {
                            return Err(self.error("vec4 requires exactly 4 arguments"));
                        }
                        Ok(Expr::Vec4(
                            Box::new(args.remove(0)),
                            Box::new(args.remove(0)),
                            Box::new(args.remove(0)),
                            Box::new(args.remove(0)),
                        ))
                    }
                    _ => Ok(Expr::Call {
                        callee: name.clone(),
                        args,
                    }),
                };
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();
        match token.token_type {
            TokenType::Int(val) => Ok(Expr::Int(val)),
            TokenType::Float(val) => Ok(Expr::Float(val)),
            TokenType::Str(val) => Ok(Expr::Str(val)),
            TokenType::Bool(val) => Ok(Expr::Bool(val)),
            TokenType::Nil => Ok(Expr::Nil),
            TokenType::Ident(name) => Ok(Expr::Variable(name)),
            TokenType::LParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(ParseError {
                message: format!("Unexpected token {:?}", token.token_type),
                line: token.line,
                column: token.column,
            }),
        }
    }

    // --- Helpers ---
    fn parse_type_name(&mut self) -> Result<String, ParseError> {
        let token = self.advance();
        match token.token_type {
            TokenType::TypeInt => Ok("int".to_string()),
            TokenType::TypeFloat => Ok("float".to_string()),
            TokenType::TypeBool => Ok("bool".to_string()),
            TokenType::TypeStr => Ok("string".to_string()),
            TokenType::TypeVec2 => Ok("vec2".to_string()),
            TokenType::TypeVec3 => Ok("vec3".to_string()),
            TokenType::TypeVec4 => Ok("vec4".to_string()),
            TokenType::Ident(name) => Ok(name),
            _ => Err(ParseError {
                message: format!("Expected type name, found {:?}", token.token_type),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn is_type_token(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::TypeInt
                | TokenType::TypeFloat
                | TokenType::TypeBool
                | TokenType::TypeStr
                | TokenType::TypeVec2
                | TokenType::TypeVec3
                | TokenType::TypeVec4
                | TokenType::Ident(_)
        )
    }

    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::EqualEqual) {
            Some(BinaryOp::Equal)
        } else if self.match_token(&TokenType::BangEqual) {
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }

    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Less) {
            Some(BinaryOp::Less)
        } else if self.match_token(&TokenType::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else if self.match_token(&TokenType::Greater) {
            Some(BinaryOp::Greater)
        } else if self.match_token(&TokenType::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else {
            None
        }
    }

    fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Plus) {
            Some(BinaryOp::Add)
        } else if self.match_token(&TokenType::Minus) {
            Some(BinaryOp::Sub)
        } else {
            None
        }
    }

    fn match_factor_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Star) {
            Some(BinaryOp::Mul)
        } else if self.match_token(&TokenType::Slash) {
            Some(BinaryOp::Div)
        } else {
            None
        }
    }

    fn match_token(&mut self, expected: &TokenType) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == expected
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn consume(&mut self, expected: &TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(self.error(msg))
        }
    }

    fn error(&self, msg: &str) -> ParseError {
        let tok = self.peek();
        ParseError {
            message: msg.to_string(),
            line: tok.line,
            column: tok.column,
        }
    }
}
