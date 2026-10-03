use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let start_col = self.column;
            let start_line = self.line;
            let ch = self.advance();

            match ch {
                // Single-character delimiters & brackets
                '(' => tokens.push(Token::new(TokenType::LParen, start_line, start_col)),
                ')' => tokens.push(Token::new(TokenType::RParen, start_line, start_col)),
                '{' => tokens.push(Token::new(TokenType::LBrace, start_line, start_col)),
                '}' => tokens.push(Token::new(TokenType::RBrace, start_line, start_col)),
                '[' => tokens.push(Token::new(TokenType::LBracket, start_line, start_col)),
                ']' => tokens.push(Token::new(TokenType::RBracket, start_line, start_col)),
                ':' => tokens.push(Token::new(TokenType::Colon, start_line, start_col)),
                ';' => tokens.push(Token::new(TokenType::Semicolon, start_line, start_col)),
                ',' => tokens.push(Token::new(TokenType::Comma, start_line, start_col)),
                '.' => tokens.push(Token::new(TokenType::Dot, start_line, start_col)),
                '+' => tokens.push(Token::new(TokenType::Plus, start_line, start_col)),
                '-' => tokens.push(Token::new(TokenType::Minus, start_line, start_col)),
                '*' => tokens.push(Token::new(TokenType::Star, start_line, start_col)),

                // Slash or comments
                '/' => {
                    if self.match_char('/') {
                        // Single-line comment: skip until newline
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else if self.match_char('*') {
                        // Multi-line comment: skip until */
                        self.skip_multiline_comment(start_line, start_col)?;
                    } else {
                        tokens.push(Token::new(TokenType::Slash, start_line, start_col));
                    }
                }

                // Operators with 1 or 2 characters
                '=' => {
                    if self.match_char('=') {
                        tokens.push(Token::new(TokenType::EqualEqual, start_line, start_col));
                    } else if self.match_char('>') {
                        tokens.push(Token::new(TokenType::FatArrow, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Equal, start_line, start_col));
                    }
                }
                '!' => {
                    if self.match_char('=') {
                        tokens.push(Token::new(TokenType::BangEqual, start_line, start_col));
                    } else {
                        return Err(LexerError {
                            message: format!("Unexpected character '!'. Did you mean '!='?"),
                            line: start_line,
                            column: start_col,
                        });
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        tokens.push(Token::new(TokenType::LessEqual, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Less, start_line, start_col));
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        tokens.push(Token::new(TokenType::GreaterEqual, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, start_line, start_col));
                    }
                }

                // String literals
                '"' => {
                    let s = self.string_literal(start_line, start_col)?;
                    tokens.push(Token::new(TokenType::Str(s), start_line, start_col));
                }

                // Numbers (int or float)
                '0'..='9' => {
                    let num_token = self.number_literal(ch, start_line, start_col);
                    tokens.push(num_token);
                }

                // Identifiers & Keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident_token = self.identifier_or_keyword(ch, start_line, start_col);
                    tokens.push(ident_token);
                }

                _ => {
                    return Err(LexerError {
                        message: format!("Unexpected character: '{}'", ch),
                        line: start_line,
                        column: start_col,
                    });
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line, self.column));
        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn skip_multiline_comment(&mut self, start_line: usize, start_col: usize) -> Result<(), LexerError> {
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                return Ok(());
            }
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }
        Err(LexerError {
            message: "Unterminated multi-line comment /* ... */".to_string(),
            line: start_line,
            column: start_col,
        })
    }

    fn string_literal(&mut self, start_line: usize, start_col: usize) -> Result<String, LexerError> {
        let mut text = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            text.push(self.advance());
        }

        if self.is_at_end() {
            return Err(LexerError {
                message: "Unterminated string literal".to_string(),
                line: start_line,
                column: start_col,
            });
        }

        self.advance(); // Consume the closing quote '"'
        Ok(text)
    }

    fn number_literal(&mut self, first_digit: char, line: usize, col: usize) -> Token {
        let mut num_str = String::new();
        num_str.push(first_digit);

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            num_str.push(self.advance());
        }

        // Check if there is a decimal point followed by digits (Float)
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            num_str.push(self.advance()); // Consume '.'
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                num_str.push(self.advance());
            }
            let val = num_str.parse::<f64>().unwrap_or(0.0);
            Token::new(TokenType::Float(val), line, col)
        } else {
            let val = num_str.parse::<i64>().unwrap_or(0);
            Token::new(TokenType::Int(val), line, col)
        }
    }

    fn identifier_or_keyword(&mut self, first_char: char, line: usize, col: usize) -> Token {
        let mut ident = String::new();
        ident.push(first_char);

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.advance());
        }

        let token_type = match ident.as_str() {
            "let" => TokenType::Let,
            "mut" => TokenType::Mut,
            "function" => TokenType::Function,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "loop" => TokenType::Loop,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "step" => TokenType::Step,
            "nil" => TokenType::Nil,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),

            // Types
            "int" => TokenType::TypeInt,
            "float" => TokenType::TypeFloat,
            "bool" => TokenType::TypeBool,
            "string" => TokenType::TypeStr,
            "vec2" => TokenType::TypeVec2,
            "vec3" => TokenType::TypeVec3,
            "vec4" => TokenType::TypeVec4,

            _ => TokenType::Ident(ident),
        };

        Token::new(token_type, line, col)
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        self.column += 1;
        ch
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
