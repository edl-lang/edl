// core/src/lexer.rs
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    String(String),
    Identifier(String),
    True,
    Type,
    False,
    Let,
    Fn,
    Return,
    If,
    Else,
    While,
    For,
    In,
    Import,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    EqEq,
    BangEq,
    Lt,
    Lte,
    Gt,
    Gte,
    Bang,
    And,
    Or,
    Dot,
    DotDot,
    Error(String),
    Eof,
    Print,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current: Option<char>,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let current = chars.next();
        Lexer { chars, current, line: 1, col: 0 }
    }

    fn advance(&mut self) -> Option<char> {
        let prev = self.current;
        if let Some(ch) = self.current {
            if ch == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
        self.current = self.chars.next();
        prev
    }

    fn peek(&self) -> Option<char> {
        self.current
    }

    pub fn next_token(&mut self) -> Token {
        use TokenKind::*;

        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => { self.advance(); continue; }
                '0'..='9' => return self.number(),
                'a'..='z' | 'A'..='Z' | '_' => return self.identifier(),
                '"' => return self.string(),

                '(' => { self.advance(); return self.single(LParen); }
                ')' => { self.advance(); return self.single(RParen); }
                '{' => { self.advance(); return self.single(LBrace); }
                '}' => { self.advance(); return self.single(RBrace); }
                ',' => { self.advance(); return self.single(Comma); }
                ';' => { self.advance(); return self.single(Semicolon); }

                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return self.single(EqEq);
                    } else {
                        return self.single(Assign);
                    }
                }

                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return self.single(BangEq);
                    } else {
                        return self.single(Bang);
                    }
                }

                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return self.single(Lte);
                    } else {
                        return self.single(Lt);
                    }
                }

                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return self.single(Gte);
                    } else {
                        return self.single(Gt);
                    }
                }

                '+' => { self.advance(); return self.single(Plus); }
                '-' => { self.advance(); return self.single(Minus); }
                '*' => { self.advance(); return self.single(Star); }
                '/' => { self.advance(); return self.single(Slash); }

                '.' => {
                    self.advance();
                    if self.peek() == Some('.') {
                        self.advance();
                        return self.single(DotDot);
                    } else {
                        return self.single(Dot);
                    }
                }

                '&' => {
                    self.advance();
                    if self.peek() == Some('&') {
                        self.advance();
                        return self.single(And);
                    } else {
                        return self.error("&".to_string());
                    }
                }

                '|' => {
                    self.advance();
                    if self.peek() == Some('|') {
                        self.advance();
                        return self.single(Or);
                    } else {
                        return self.error("|".to_string());
                    }
                }

                other => {
                    let c = other.to_string();
                    self.advance();
                    return self.error(c);
                }
            }
        }

        Token { kind: Eof, line: self.line, col: self.col }
    }

    fn single(&self, kind: TokenKind) -> Token {
        Token { kind, line: self.line, col: self.col }
    }

    fn error(&self, c: String) -> Token {
        Token { kind: TokenKind::Error(c), line: self.line, col: self.col }
    }

    fn number(&mut self) -> Token {
        let mut num = String::new();
        let start_col = self.col;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                self.advance();
            } else { break; }
        }
        let value = num.parse::<f64>().unwrap_or(0.0);
        Token { kind: TokenKind::Number(value), line: self.line, col: start_col }
    }

    fn identifier(&mut self) -> Token {
        let mut ident = String::new();
        let start_col = self.col;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else { break; }
        }
        let kind = match ident.as_str() {
            "true"   => TokenKind::True,
            "false"  => TokenKind::False,
            "let"    => TokenKind::Let,
            "fn"     => TokenKind::Fn,
            "return" => TokenKind::Return,
            "if"     => TokenKind::If,
            "else"   => TokenKind::Else,
            "while"  => TokenKind::While,
            "for"    => TokenKind::For,
            "in"     => TokenKind::In,
            "import" => TokenKind::Import,
            _        => TokenKind::Identifier(ident),
        };
        Token { kind, line: self.line, col: start_col }
    }

    fn string(&mut self) -> Token {
        let mut s = String::new();
        let start_col = self.col;
        self.advance(); // Skip opening "
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                break;
            } else {
                s.push(c);
                self.advance();
            }
        }
        Token { kind: TokenKind::String(s), line: self.line, col: start_col }
    }
}