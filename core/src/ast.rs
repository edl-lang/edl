// core/src/ast.rs

/// Expressions du langage EDL
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),                        // Literal number
    Bool(bool),                         // Literal boolean
    String(String),                     // Literal string
    Variable(String),                   // Variable reference
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> }, // Binary operation
    Unary { op: UnOp, expr: Box<Expr> },                     // Unary operation
    Call { function: Box<Expr>, arguments: Vec<Expr> },      // Function call
    Assign { name: String, expr: Box<Expr> },                // Assignment
    Block(Vec<Stmt>),                   // Block of statements
    Lambda { params: Vec<String>, body: Vec<Stmt> },         // Anonymous function
    List(Vec<Expr>),                    // List literal
    Dict(Vec<(Expr, Expr)>),            // Dictionary literal
    Tuple(Vec<Expr>),                   // Tuple literal (optional)
    Annotated(Box<Expr>, String),       // Type annotation (optional)
    Invalid(String),                    // Invalid expression (for error recovery)
    FieldAccess { object: Box<Expr>, field: String },       // Field access
    Instance { type_name: String, fields: Vec<(String, Expr)> }, // Ajoute ceci
}

/// Instructions/statements du langage EDL
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),                         // Standalone expression
    Let { name: String, expr: Expr },   // Variable declaration
    Const { name: String, expr: Expr }, // Constant declaration
    Function { name: String, params: Vec<String>, body: Vec<Stmt> }, // Function definition
    Return(Option<Expr>),               // Return statement
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> }, // If/else
    While { condition: Expr, body: Vec<Stmt> }, // While loop
    For { var: String, start: Expr, end: Expr, body: Vec<Stmt> }, // For loop
    Import(String),                     // Import statement
    Block(Vec<Stmt>),                   // Block statement
    Break,                              // Break statement
    Continue,                           // Continue statement
    Print(Expr),                        // Print statement
    PrintArgs(Vec<Expr>),               // Ajoute cette variante
    Type { name: String, fields: Vec<(String, Expr)>, methods: Vec<Stmt> }, // Custom type
    Struct { name: String, fields: Vec<(String, Expr)> }, // Struct
    Enum { name: String, variants: Vec<String> },         // Enum
    Match { expr: Expr, arms: Vec<(Expr, Vec<Stmt>)> },   // Pattern matching
    Invalid(String),                    // Invalid statement (for error recovery)
    NativeFunction { name: String, params: Vec<String>, body: Vec<Stmt> }, // Native function
}

/// Opérateurs binaires
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Pow, 
    Mod, // Ajoute l'opérateur modulo
    Concat, // Ajoute l'opérateur de concaténation pour les chaînes
    In, // Ajoute l'opérateur d'appartenance pour les listes/dicts
    Contains, // Ajoute l'opérateur de vérification de présence dans les listes/dicts
    FieldAccess, // Ajoute l'opérateur d'accès aux champs pour les objets
    InstanceOf, // Ajoute l'opérateur de vérification d'instance pour les types
    BitAnd, // Ajoute l'opérateur ET binaire
    BitOr,  // Ajoute l'opérateur OU binaire
    BitXor, // Ajoute l'opérateur XOR binaire
    BitShiftLeft, // Ajoute l'opérateur de décalage à gauche
    BitShiftRight, // Ajoute l'opérateur de décalage à droite
    BitNot, // Ajoute l'opérateur de complément binaire
    Range, // Ajoute l'opérateur de plage pour les boucles for
    RangeInclusive, // Ajoute l'opérateur de plage inclusive pour les boucles for   
}

/// Opérateurs unaires
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg, Not,
}