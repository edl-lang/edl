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
    Const,
    Match,
    As,
    Pub,
    Mod,
    Struct,
    Enum,
    Break,
    Continue,
    Yield,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current: Option<char>,
    pub line: usize,
    pub col: usize,
    peeked: Option<Option<char>>, // cache pour peek() pour ne pas avancer
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let current = chars.next();
        Lexer {
            chars,
            current,
            line: 1,
            col: 0,
            peeked: None,
        }
    }

    /// Avance d’un caractère et retourne l’ancien caractère
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
        self.peeked = None; // invalider cache peek
        prev
    }

    /// Regarde le caractère courant sans avancer
    fn peek(&mut self) -> Option<char> {
        if let Some(cached) = self.peeked {
            cached
        } else {
            let c = self.current;
            self.peeked = Some(c);
            c
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        use TokenKind::*;

        // Ignorer les espaces blancs
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                    continue;
                }
                '0'..='9' => return self.number(),
                'a'..='z' | 'A'..='Z' | '_' => return Ok(self.identifier()),
                '"' => return self.string(), // <-- Propagation directe de l'erreur

                '(' => { self.advance(); return Ok(self.single(LParen)); }
                ')' => { self.advance(); return Ok(self.single(RParen)); }
                '{' => { self.advance(); return Ok(self.single(LBrace)); }
                '}' => { self.advance(); return Ok(self.single(RBrace)); }
                ',' => { self.advance(); return Ok(self.single(Comma)); }
                ';' => { self.advance(); return Ok(self.single(Semicolon)); }

                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Ok(self.single(EqEq));
                    } else {
                        return Ok(self.single(Assign));
                    }
                }

                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Ok(self.single(BangEq));
                    } else {
                        return Ok(self.single(Bang));
                    }
                }

                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Ok(self.single(Lte));
                    } else {
                        return Ok(self.single(Lt));
                    }
                }

                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Ok(self.single(Gte));
                    } else {
                        return Ok(self.single(Gt));
                    }
                }

                '+' => { self.advance(); return Ok(self.single(Plus)); }
                '-' => { self.advance(); return Ok(self.single(Minus)); }
                '*' => { self.advance(); return Ok(self.single(Star)); }
                '/' => {
                    self.advance();
                    if self.peek() == Some('/') {
                        // Commentaire ligne
                        while let Some(c) = self.peek() {
                            if c == '\n' { break; }
                            self.advance();
                        }
                        return self.next_token();
                    } else {
                        return Ok(self.single(Slash));
                    }
                }

                '.' => {
                    self.advance();
                    if self.peek() == Some('.') {
                        self.advance();
                        return Ok(self.single(DotDot));
                    } else {
                        return Ok(self.single(Dot));
                    }
                }

                '&' => {
                    self.advance();
                    if self.peek() == Some('&') {
                        self.advance();
                        return Ok(self.single(And));
                    } else {
                        return Err(LexError {
                            message: "Unexpected character '&' (did you mean '&&'?)".to_string(),
                            line: self.line,
                            col: self.col,
                        });
                    }
                }

                '|' => {
                    self.advance();
                    if self.peek() == Some('|') {
                        self.advance();
                        return Ok(self.single(Or));
                    } else {
                        return Err(LexError {
                            message: "Unexpected character '|' (did you mean '||'?)".to_string(),
                            line: self.line,
                            col: self.col,
                        });
                    }
                }

                '#' => {
                    // Commentaire ligne style Python
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                    return self.next_token();
                }

                other => {
                    let c = other.to_string();
                    self.advance();
                    // En cas d'erreur :
                    return Err(LexError {
                        message: format!("Unexpected character '{}'", c),
                        line: self.line,
                        col: self.col,
                    });
                }
            }
        }

        Ok(Token { kind: Eof, line: self.line, col: self.col })
    }

    fn single(&self, kind: TokenKind) -> Token {
        Token { kind, line: self.line, col: self.col }
    }

    fn number(&mut self) -> Result<Token, LexError> {
        let mut num = String::new();
        let start_col = self.col;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                self.advance();
            } else { break; }
        }
        match num.parse::<f64>() {
            Ok(value) => Ok(Token { kind: TokenKind::Number(value), line: self.line, col: start_col }),
            Err(_) => Err(LexError {
                message: format!("Invalid number literal '{}'", num),
                line: self.line,
                col: start_col,
            }),
        }
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
            "print" => TokenKind::Print,
            "type"   => TokenKind::Type,
            "const"  => TokenKind::Const,
            "match"  => TokenKind::Match,
            "as"     => TokenKind::As,
            "pub"    => TokenKind::Pub,
            "mod"    => TokenKind::Mod,
            "struct" => TokenKind::Struct,
            "enum"   => TokenKind::Enum,
            "break"  => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "yield"  => TokenKind::Yield,
            _        => TokenKind::Identifier(ident),
        };
        Token { kind, line: self.line, col: start_col }
    }

    fn string(&mut self) -> Result<Token, LexError> {
        let mut s = String::new();
        let start_col = self.col;
        self.advance(); // Skip opening "

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // skip closing "
                return Ok(Token { kind: TokenKind::String(s), line: self.line, col: start_col });
            } else if c == '\\' {
                self.advance(); // skip '\'
                match self.peek() {
                    Some('n') => { s.push('\n'); self.advance(); }
                    Some('r') => { s.push('\r'); self.advance(); }
                    Some('t') => { s.push('\t'); self.advance(); }
                    Some('"') => { s.push('"'); self.advance(); }
                    Some('\\') => { s.push('\\'); self.advance(); }
                    Some(other) => { s.push(other); self.advance(); }
                    None => break,
                }
            } else {
                s.push(c);
                self.advance();
            }
        }

        // Si la chaîne n'est pas terminée :
        Err(LexError {
            message: "Unterminated string literal".to_string(),
            line: self.line,
            col: start_col,
        })
    }
}