// core/src/parser.rs

// --- Dépendances et définitions de base ---
use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};

/// Erreurs possibles lors du parsing
#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnexpectedEof,
    Custom,
}

/// Le parser EDL, qui transforme une suite de tokens en AST
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Token,
}

impl<'a> Parser<'a> {
    /// Crée un nouveau parser à partir d'une source
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer::new(src);
        let curr = lexer.next_token().unwrap_or_else(|err| Token {
            kind: TokenKind::Error(err.message),
            line: err.line,
            col: err.col,
        });
        Parser { lexer, curr }
    }

    /// Passe au token suivant
    fn advance(&mut self) {
        match self.lexer.next_token() {
            Ok(token) => self.curr = token,
            Err(err) => {
                self.curr = Token {
                    kind: TokenKind::Error(err.message.clone()),
                    line: err.line,
                    col: err.col,
                };
            }
        }
    }

    /// Vérifie et consomme le token attendu
    fn expect(&mut self, kind: &TokenKind) -> Result<(), ParseError> {
        if &self.curr.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: format!("Expected {:?}, found {:?}", kind, self.curr.kind),
                line: self.curr.line,
                col: self.curr.col,
            })
        }
    }

    /// Génère une erreur UnexpectedToken avec contexte et exemple
    fn unexpected(&self, context: &str, expected: &str) -> ParseError {
        ParseError {
            kind: ParseErrorKind::UnexpectedToken,
            message: format!(
                "Erreur de syntaxe {} : attendu {}, trouvé {:?} à la ligne {}, colonne {}.",
                context, expected, self.curr.kind, self.curr.line, self.curr.col
            ),
            line: self.curr.line,
            col: self.curr.col,
        }
    }

    /// Génère une erreur Custom avec message et contexte
    #[allow(dead_code)]
    fn custom_error(&self, context: &str, msg: &str) -> ParseError {
        ParseError {
            kind: ParseErrorKind::Custom,
            message: format!("{} : {} (ligne {}, colonne {})", context, msg, self.curr.line, self.curr.col),
            line: self.curr.line,
            col: self.curr.col,
        }
    }

    // =========================
    //   Entrée principale
    // =========================

    /// Parse tout le fichier/source en une liste d'instructions (statements)
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while self.curr.kind != TokenKind::Eof {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    // =========================
    //   Parsing des statements
    // =========================

    /// Parse une instruction (statement)
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match &self.curr.kind {
            TokenKind::Type => self.parse_type(),
            TokenKind::Let => self.parse_let(),
            TokenKind::Fn => self.parse_function(),
            TokenKind::Const => self.parse_const(),                // Ajoute cette ligne
            TokenKind::Native => self.parse_native_function(),     // Ajoute cette ligne
            TokenKind::Return => self.parse_return(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(),
            TokenKind::Import => self.parse_import(),
            TokenKind::Break => self.parse_break(),
            TokenKind::Continue => self.parse_continue(),
            TokenKind::Print => self.parse_print(),
            TokenKind::LBrace => self.parse_block_stmt(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Enum => self.parse_enum(),
            // Expression seule (ex: appel de fonction, affectation, etc.)
            _ => {
                let expr = self.parse_expr()?;
                if self.curr.kind == TokenKind::Semicolon {
                    self.advance();
                }
                Ok(Stmt::Expr(expr))
            }
        }
    }

    /// let x = ...;
    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Let)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(self.unexpected("dans la déclaration de variable", "un identifiant"));
        };
        self.advance();
        // Ici, il ne faut PAS exiger un TokenKind::Colon
        if self.curr.kind == TokenKind::Colon {
            self.advance();
            // ...parse type annotation...
        }
        self.expect(&TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Let { name, expr })
    }

    /// fn nom(...) { ... }
    fn parse_function(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Fn)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: "Expected function name".to_string(),
                line: self.curr.line,
                col: self.curr.col,
            });
        };
        self.advance();
        self.expect(&TokenKind::LParen)?;

        // Paramètres de la fonction
        let mut params = Vec::new();
        while self.curr.kind != TokenKind::RParen {
            if let TokenKind::Identifier(param) = &self.curr.kind {
                params.push(param.clone());
                self.advance();
                if self.curr.kind == TokenKind::Comma {
                    self.advance();
                } else if self.curr.kind != TokenKind::RParen {
                    return Err(self.unexpected("dans la liste des paramètres", "une virgule ou une parenthèse fermante"));
                }
            } else {
                return Err(self.unexpected("dans la liste des paramètres", "un identifiant"));
            }
        }
        self.expect(&TokenKind::RParen)?;

        // Ici, accepte un ':' optionnel avant '{'
        if self.curr.kind == TokenKind::Colon {
            self.advance();
        }
        self.expect(&TokenKind::LBrace)?;

        let start_line = self.curr.line;
        let mut body = Vec::new();
        while self.curr.kind != TokenKind::RBrace && self.curr.kind != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RBrace)?;
        let end_line = self.curr.line;

        if end_line - start_line > 64 {
            return Err(ParseError {
                kind: ParseErrorKind::Custom,
                message: format!("Function '{}' exceeds 64 lines ({} lines)", name, end_line - start_line),
                line: start_line,
                col: self.curr.col,
            });
        }

        Ok(Stmt::Function { name, params, body })
    }

    /// return ...;
    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Return)?;
        let expr = if self.curr.kind != TokenKind::Semicolon {
            Some(self.parse_expr()?)
        } else {
            None
        };
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Return(expr))
    }

    /// if ... { ... } [else if ... { ... }] [else { ... }]
    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::If)?;
        let condition = self.parse_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.curr.kind == TokenKind::Else {
            self.advance();
            if self.curr.kind == TokenKind::If {
                // else if ... => parse un autre if récursivement
                Some(vec![self.parse_if()?])
            } else {
                // else { ... }
                Some(self.parse_block()?)
            }
        } else {
            None
        };
        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    /// while ... { ... }
    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::While)?;
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    /// for i in ... { ... }
    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::For)?;
        let var = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: format!("Token inattendu : {:?}", self.curr),
                line: self.curr.line,
                col: self.curr.col,
            });
        };
        self.advance();
        self.expect(&TokenKind::In)?;
        let start = self.parse_expr()?;
        self.expect(&TokenKind::DotDot)?;
        let end = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::For { var, start, end, body })
    }

    /// import "fichier";
    fn parse_import(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Import)?;
        let path = if let TokenKind::String(path) = &self.curr.kind {
            path.clone()
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: format!("Token inattendu : {:?}", self.curr),
                line: self.curr.line,
                col: self.curr.col,
            });
        };
        self.advance();
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Import(path))
    }

    /// break;
    fn parse_break(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Break)?;
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Break)
    }

    /// continue;
    fn parse_continue(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Continue)?;
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Continue)
    }

    /// print ...;
    fn parse_print(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Print)?;
        let mut args = Vec::new();
        args.push(self.parse_expr()?);
        while self.curr.kind == TokenKind::Comma {
            self.advance();
            args.push(self.parse_expr()?);
        }
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::PrintArgs(args))
    }

    /// { ... }
    fn parse_block_stmt(&mut self) -> Result<Stmt, ParseError> {
        Ok(Stmt::Block(self.parse_block()?))
    }

    /// Parse le contenu d'un bloc { ... }
    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(&TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while self.curr.kind != TokenKind::RBrace && self.curr.kind != TokenKind::Eof {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(stmts)
    }

    // =========================
    //   Parsing des expressions
    // =========================

    /// Expression complète (affectation ou logique)
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    /// Affectation : x = ...
    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_logic_or()?;
        if self.curr.kind == TokenKind::Assign {
            self.advance();
            let value = self.parse_assignment()?;
            if let Expr::Variable(name) = expr {
                Ok(Expr::Assign { name, expr: Box::new(value) })
            } else {
                Err(ParseError {
                    kind: ParseErrorKind::Custom,
                    message: "Cible d'affectation invalide".to_string(),
                    line: self.curr.line,
                    col: self.curr.col,
                })
            }
        } else {
            Ok(expr)
        }
    }

    /// Opérateur logique OR
    fn parse_logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logic_and()?;
        while self.curr.kind == TokenKind::Or {
            self.advance();
            let right = self.parse_logic_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Opérateur logique AND
    fn parse_logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;
        while self.curr.kind == TokenKind::And {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Comparaisons ==, !=
    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        while matches!(self.curr.kind, TokenKind::EqEq | TokenKind::BangEq) {
            let op = match &self.curr.kind {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::BangEq => BinOp::Neq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Comparaisons <, <=, >, >=
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        while matches!(self.curr.kind, TokenKind::Lt | TokenKind::Lte | TokenKind::Gt | TokenKind::Gte) {
            let op = match &self.curr.kind {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Lte => BinOp::Lte,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Gte => BinOp::Gte,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Addition/Soustraction
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        while matches!(self.curr.kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match &self.curr.kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Multiplication/Division
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_power()?;
        while matches!(self.curr.kind, TokenKind::Star | TokenKind::Slash) {
            let op = match &self.curr.kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Puissance (^)
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        while self.curr.kind == TokenKind::Pow {
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinOp::Pow,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Unaires (-, !)
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match &self.curr.kind {
            TokenKind::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnOp::Neg,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            TokenKind::Bang => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnOp::Not,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_call(),
        }
    }

    /// Appels de fonction et chainage
    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.curr.kind == TokenKind::LParen {
                self.advance();
                let mut args = Vec::new();
                if self.curr.kind != TokenKind::RParen {
                    args.push(self.parse_expr()?);
                    while self.curr.kind == TokenKind::Comma {
                        self.advance();
                        args.push(self.parse_expr()?);
                    }
                }
                self.expect(&TokenKind::RParen)?;
                expr = Expr::Call {
                    function: Box::new(expr),
                    arguments: args,
                };
            } else if self.curr.kind == TokenKind::Dot {
                self.advance();
                if let TokenKind::Identifier(field) = &self.curr.kind {
                    let field_name = field.clone();
                    self.advance();
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field: field_name,
                    };
                } else {
                    // erreur
                }
            } else if self.curr.kind == TokenKind::LBracket {
                self.advance();
                let index = self.parse_expr()?;
                self.expect(&TokenKind::RBracket)?;
                expr = Expr::Index {
                    collection: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// Valeurs primaires (littéraux, variables, parenthèses, blocs, lambdas)
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match &self.curr.kind {
            TokenKind::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }
            TokenKind::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::String(val))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenKind::Identifier(name) => {
                let var = name.clone();
                self.advance();
                // NE PAS traiter ici l'appel de fonction ou l'indexation
                // Juste retourner la variable, le chainage (appel, index, etc.) sera géré dans parse_call
                Ok(Expr::Variable(var))
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if self.curr.kind != TokenKind::RBracket {
                    elements.push(self.parse_expr()?);
                    while self.curr.kind == TokenKind::Comma {
                        self.advance();
                        elements.push(self.parse_expr()?);
                    }
                }
                self.expect(&TokenKind::RBracket)?;
                Ok(Expr::List(elements))
            }
            TokenKind::LBrace => {
                // Dictionnaire (clé: valeur, ...)
                self.advance();
                let mut entries = Vec::new();
                if self.curr.kind != TokenKind::RBrace {
                    loop {
                        let key = self.parse_expr()?;
                        self.expect(&TokenKind::Colon)?;
                        let value = self.parse_expr()?;
                        entries.push((key, value));
                        if self.curr.kind == TokenKind::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RBrace)?;
                Ok(Expr::Dict(entries))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if self.curr.kind == TokenKind::Comma {
                    // Tuple
                    let mut elements = vec![expr];
                    while self.curr.kind == TokenKind::Comma {
                        self.advance();
                        elements.push(self.parse_expr()?);
                    }
                    self.expect(&TokenKind::RParen)?;
                    Ok(Expr::Tuple(elements))
                } else {
                    self.expect(&TokenKind::RParen)?;
                    Ok(expr)
                }
            }
            TokenKind::Lambda => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let mut params = Vec::new();
                while self.curr.kind != TokenKind::RParen {
                    if let TokenKind::Identifier(param) = &self.curr.kind {
                        params.push(param.clone());
                        self.advance();
                        if self.curr.kind == TokenKind::Comma {
                            self.advance();
                        } else if self.curr.kind != TokenKind::RParen {
                            return Err(ParseError {
                                kind: ParseErrorKind::UnexpectedToken,
                                message: format!("Token inattendu : {:?}", self.curr),
                                line: self.curr.line,
                                col: self.curr.col,
                            });
                        }
                    } else {
                        return Err(ParseError {
                            kind: ParseErrorKind::UnexpectedToken,
                            message: format!("Token inattendu : {:?}", self.curr),
                            line: self.curr.line,
                            col: self.curr.col,
                        });
                    }
                }
                self.expect(&TokenKind::RParen)?;
                let body = if self.curr.kind == TokenKind::LBrace {
                    self.advance();
                    let mut stmts = Vec::new();
                    while self.curr.kind != TokenKind::RBrace && self.curr.kind != TokenKind::Eof {
                        stmts.push(self.parse_stmt()?);
                    }
                    self.expect(&TokenKind::RBrace)?;
                    Stmt::Block(stmts)
                } else {
                    self.parse_stmt()?
                };
                Ok(Expr::Lambda { params, body: vec![body] })
            }
            _ => Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: format!("Token inattendu : {:?}", self.curr),
                line: self.curr.line,
                col: self.curr.col,
            }),
        }
    }

    /// Parse un type : `type Nom { ... }`
    fn parse_type(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Type)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: "Expected type name".to_string(),
                line: self.curr.line,
                col: self.curr.col,
            });
        };
        self.advance();
        self.expect(&TokenKind::LBrace)?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while self.curr.kind != TokenKind::RBrace && self.curr.kind != TokenKind::Eof {
            if self.curr.kind == TokenKind::Fn {
                methods.push(self.parse_function()?);
            } else if let TokenKind::Identifier(field_name) = &self.curr.kind {
                let field = field_name.clone();
                self.advance();
                self.expect(&TokenKind::Colon)?;
                let expr = self.parse_expr()?;
                if self.curr.kind == TokenKind::Comma {
                    self.advance();
                }
                fields.push((field, expr));
            } else {
                return Err(ParseError {
                    kind: ParseErrorKind::UnexpectedToken,
                    message: format!("Unexpected token in type body: {:?}", self.curr.kind),
                    line: self.curr.line,
                    col: self.curr.col,
                });
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(Stmt::Type { name, fields, methods })
    }

    /// const x = ...;
    fn parse_const(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Const)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(self.unexpected("dans la déclaration de constante", "un identifiant"));
        };
        self.advance();
        self.expect(&TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        if self.curr.kind == TokenKind::Semicolon {
            self.advance();
        }
        Ok(Stmt::Const { name, expr })
    }

    /// native x = ...;
    fn parse_native_function(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Native)?;
        self.expect(&TokenKind::Fn)?;
        let name = if let TokenKind::Identifier(name) = &self.curr.kind {
            name.clone()
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken,
                message: "Expected native function name".to_string(),
                line: self.curr.line,
                col: self.curr.col,
            });
        };
        self.advance();
        self.expect(&TokenKind::LParen)?;

        // Paramètres de la fonction native
        let mut params = Vec::new();
        while self.curr.kind != TokenKind::RParen {
            if let TokenKind::Identifier(param) = &self.curr.kind {
                params.push(param.clone());
                self.advance();
                if self.curr.kind == TokenKind::Comma {
                    self.advance();
                } else if self.curr.kind != TokenKind::RParen {
                    return Err(self.unexpected("dans la liste des paramètres de la fonction native", "une virgule ou une parenthèse fermante"));
                }
            } else {
                return Err(self.unexpected("dans la liste des paramètres de la fonction native", "un identifiant"));
            }
        }
        self.expect(&TokenKind::RParen)?;

        // Ici, accepte un ':' optionnel avant '{'
        if self.curr.kind == TokenKind::Colon {
            self.advance();
        }
        self.expect(&TokenKind::LBrace)?;

        let start_line = self.curr.line;
        let mut body = Vec::new();
        while self.curr.kind != TokenKind::RBrace && self.curr.kind != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(&TokenKind::RBrace)?;
        let end_line = self.curr.line;

        if end_line - start_line > 64 {
            return Err(ParseError {
                kind: ParseErrorKind::Custom,
                message: format!("Native function '{}' exceeds 64 lines ({} lines)", name, end_line - start_line),
                line: start_line,
                col: self.curr.col,
            });
        }

        Ok(Stmt::NativeFunction { name, params, body })
    }

    /// match ... { ... }
    fn parse_match(&mut self) -> Result<Stmt, ParseError> {
        Err(ParseError {
            kind: ParseErrorKind::Custom,
            message: "parse_match n'est pas encore implémenté".to_string(),
            line: self.curr.line,
            col: self.curr.col,
        })
    }

    /// enum ... { ... }
    fn parse_enum(&mut self) -> Result<Stmt, ParseError> {
        Err(ParseError {
            kind: ParseErrorKind::Custom,
            message: "parse_enum n'est pas encore implémenté".to_string(),
            line: self.curr.line,
            col: self.curr.col,
        })
    }
}