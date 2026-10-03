#![allow(warnings)]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Let, If, Else, While, Loop, Break, Step, Fn, Return, True, False, Nil,
    Print, Println, Input, ReadFile, WriteFile,
    Vec2, Vec3, Vec4,
    Ident(String), Int(i64), Float(f64), Str(String),
    Plus, Minus, Star, Slash, Percent, Assign,
    EqualEqual, BangEqual, Less, LessEqual, Greater, GreaterEqual, AndAnd, OrOr, Bang,
    LParen, RParen, LBrace, RBrace, Comma, Semicolon,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self { token_type, line, column }
    }
}
