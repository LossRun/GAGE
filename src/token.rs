#![allow(warnings)]

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Declarations & OOP
    Let,
    Fn,
    Return,
    Class,
    New,
    This,

    // Control Flow
    If,
    Else,
    While,
    For,
    In,
    Loop,
    Break,
    Step,

    // Literals & Primitives
    True,
    False,
    Nil,
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),

    // Vector Types
    Vec2,
    Vec3,
    Vec4,

    // Built-in Functions
    Print,
    Println,
    Input,
    ReadFile,
    Random,
    Clamp,
    ParseInt,
    ClearScreen,
    Lerp,
    Distance,
    Reflect,
    WriteFile,
    Dot,
    Cross,
    Length,
    Normalize,

    // Operators & Delimiters
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    Bang,
    DotOp,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,

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
