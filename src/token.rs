#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Ident(String),

    // Keywords
    Let,
    Mut,
    Function,
    Return,
    If,
    Else,
    While,
    Loop,
    Break,
    Continue,
    Step,     // Simulation tick construct: step(dt) { ... }
    Nil,

    // Built-in Primitive & Vector Types
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeStr,
    TypeVec2,
    TypeVec3,
    TypeVec4,

    // Operators
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Equal,         // =
    EqualEqual,    // ==
    BangEqual,     // !=
    Less,          // <
    LessEqual,     // <=
    Greater,       // >
    GreaterEqual,  // >=
    FatArrow,      // =>

    // Delimiters & Punctuation
    Colon,         // :
    Semicolon,     // ;
    Comma,         // ,
    Dot,           // .
    LParen,        // (
    RParen,        // )
    LBrace,        // {
    RBrace,        // }
    LBracket,      // [
    RBracket,      // ]

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}
