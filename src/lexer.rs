#![allow(warnings)]
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct LexError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> { self.chars.get(self.pos).copied() }
    fn peek_next(&self) -> Option<char> { self.chars.get(self.pos + 1).copied() }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += 1;
            if ch == '\n' { self.line += 1; self.col = 1; } else { self.col += 1; }
            Some(ch)
        } else { None }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            let start_line = self.line;
            let start_col = self.col;

            match ch {
                ' ' | '\t' | '\r' | '\n' => { self.advance(); }
                '/' if self.peek_next() == Some('/') => {
                    while let Some(c) = self.peek() {
                        self.advance();
                        if c == '\n' { break; }
                    }
                }
                '+' => { self.advance(); tokens.push(Token::new(TokenType::Plus, start_line, start_col)); }
                '-' => { self.advance(); tokens.push(Token::new(TokenType::Minus, start_line, start_col)); }
                '*' => { self.advance(); tokens.push(Token::new(TokenType::Star, start_line, start_col)); }
                '/' => { self.advance(); tokens.push(Token::new(TokenType::Slash, start_line, start_col)); }
                '%' => { self.advance(); tokens.push(Token::new(TokenType::Percent, start_line, start_col)); }
                '(' => { self.advance(); tokens.push(Token::new(TokenType::LParen, start_line, start_col)); }
                ')' => { self.advance(); tokens.push(Token::new(TokenType::RParen, start_line, start_col)); }
                '{' => { self.advance(); tokens.push(Token::new(TokenType::LBrace, start_line, start_col)); }
                '}' => { self.advance(); tokens.push(Token::new(TokenType::RBrace, start_line, start_col)); }
                '[' => { self.advance(); tokens.push(Token::new(TokenType::LBracket, start_line, start_col)); }
                ']' => { self.advance(); tokens.push(Token::new(TokenType::RBracket, start_line, start_col)); }
                ',' => { self.advance(); tokens.push(Token::new(TokenType::Comma, start_line, start_col)); }
                ';' => { self.advance(); tokens.push(Token::new(TokenType::Semicolon, start_line, start_col)); }
                '.' => { self.advance(); tokens.push(Token::new(TokenType::DotOp, start_line, start_col)); }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::EqualEqual, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Assign, start_line, start_col));
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::BangEqual, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Bang, start_line, start_col));
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::LessEqual, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Less, start_line, start_col));
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::GreaterEqual, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, start_line, start_col));
                    }
                }
                '&' if self.peek_next() == Some('&') => {
                    self.advance(); self.advance();
                    tokens.push(Token::new(TokenType::AndAnd, start_line, start_col));
                }
                '|' if self.peek_next() == Some('|') => {
                    self.advance(); self.advance();
                    tokens.push(Token::new(TokenType::OrOr, start_line, start_col));
                }
                '"' => {
                    self.advance();
                    let mut s = String::new();
                    let mut closed = false;
                    while let Some(c) = self.peek() {
                        self.advance();
                        if c == '"' { closed = true; break; }
                        if c == '\\' {
                            if let Some(esc) = self.advance() {
                                match esc {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    '\\' => s.push('\\'),
                                    '"' => s.push('"'),
                                    _ => s.push(esc),
                                }
                            }
                        } else { s.push(c); }
                    }
                    if !closed {
                        return Err(LexError { line: start_line, column: start_col, message: "Unterminated string literal".into() });
                    }
                    tokens.push(Token::new(TokenType::Str(s), start_line, start_col));
                }
                ch if ch.is_ascii_digit() => {
                    let mut num_str = String::new();
                    let mut is_float = false;
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(c);
                            self.advance();
                        } else if c == '.' && !is_float && self.peek_next().map_or(false, |n| n.is_ascii_digit()) {
                            is_float = true;
                            num_str.push(c);
                            self.advance();
                        } else { break; }
                    }
                    if is_float {
                        tokens.push(Token::new(TokenType::Float(num_str.parse().unwrap()), start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Int(num_str.parse().unwrap()), start_line, start_col));
                    }
                }
                ch if ch.is_alphabetic() || ch == '_' => {
                    let mut ident = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '_' { ident.push(c); self.advance(); }
                        else { break; }
                    }
                    let token_type = match ident.as_str() {
                        "let" => TokenType::Let,
                        "fn" => TokenType::Fn,
                        "return" => TokenType::Return,
                        "class" => TokenType::Class,
                        "new" => TokenType::New,
                        "this" => TokenType::This,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "for" => TokenType::For,
                        "in" => TokenType::In,
                        "loop" => TokenType::Loop,
                        "break" => TokenType::Break,
                        "step" => TokenType::Step,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "nil" => TokenType::Nil,
                        "print" => TokenType::Print,
                        "println" => TokenType::Println,
                        "input" => TokenType::Input,
                        "read_file" => TokenType::ReadFile,
                        "write_file" => TokenType::WriteFile,
                        "dot" => TokenType::Dot,
                        "cross" => TokenType::Cross,
                        "length" => TokenType::Length,
                        "normalize" => TokenType::Normalize,
                        "vec2" => TokenType::Vec2,
                        "vec3" => TokenType::Vec3,
                        "vec4" => TokenType::Vec4,
                        _ => TokenType::Ident(ident),
                    };
                    tokens.push(Token::new(token_type, start_line, start_col));
                }
                _ => return Err(LexError { line: start_line, column: start_col, message: format!("Unexpected character: '{}'", ch) }),
            }
        }

        tokens.push(Token::new(TokenType::EOF, self.line, self.col));
        Ok(tokens)
    }
}
