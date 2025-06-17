// core/src/ast.rs

/// Expressions du langage EDL
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),                        // Literal number
    Bool(bool),                        // Literal boolean
    String(String),                   // Literal string
    Variable(String),                 // Variable reference
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },                               // Binary operation
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },                               // Unary operation
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },                               // Function call
    Assign {
        name: String,
        expr: Box<Expr>,
    },                               // Assignment
    Block(Vec<Stmt>),                // Block of statements
    Lambda {
        params: Vec<String>,
        body: Vec<Stmt>,
    },                               // Anonymous function
    List(Vec<Expr>),                // List literal
    Dict(Vec<(Expr, Expr)>),        // Dictionary literal
    Tuple(Vec<Expr>),               // Tuple literal
    Annotated(Box<Expr>, String),  // Type annotation (optional)
    Invalid(String),               // Invalid expression (for error recovery)
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },                             // Field access
    Instance {
        type_name: String,
        fields: Vec<(String, Expr)>,
    },                             // Instance creation
    Index {
        collection: Box<Expr>,
        index: Box<Expr>,
    },                             // Indexation (list/dict)
    Await(Box<Expr>),              // Await expression (async)
    Yield(Option<Box<Expr>>),     // Yield expression for generators
    MatchExpr {
        expr: Box<Expr>,
        arms: Vec<(Expr, Expr)>,  // Pattern matching as expression
    },
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },                             // Ternary conditional expression
    InterpolatedString(Vec<Expr>),// String interpolation
}

/// Instructions/statements du langage EDL
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),                   // Expression statement
    Let {
        name: String,
        expr: Expr,
    },                            // Variable declaration
    Const {
        name: String,
        expr: Expr,
    },                            // Constant declaration
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },                            // Function definition
    Return(Option<Expr>),         // Return statement
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },                            // If/else statement
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },                            // While loop
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Vec<Stmt>,
    },                            // For loop
    Import(String),               // Import statement
    ImportAlias {
        path: String,
        alias: String,
    },                            // Import with alias
    Block(Vec<Stmt>),             // Block statement
    Break,                       // Break statement
    Continue,                    // Continue statement
    Print(Expr),                 // Print statement
    PrintArgs(Vec<Expr>),        // Print with multiple arguments
    Type {
        name: String,
        fields: Vec<(String, Expr)>,
        methods: Vec<Stmt>,
    },                            // Custom type
    Struct {
        name: String,
        fields: Vec<(String, Expr)>,
    },                            // Struct
    Enum {
        name: String,
        variants: Vec<String>,
    },                            // Enum
    Match {
        expr: Expr,
        arms: Vec<(Expr, Vec<Stmt>)>,
    },                            // Pattern matching statement
    TryCatch {
        try_block: Vec<Stmt>,
        catch_param: String,
        catch_block: Vec<Stmt>,
    },                            // Try/catch statement
    Defer(Vec<Stmt>),            // Defer statement
    Switch {
        expr: Expr,
        cases: Vec<(Expr, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
    },                            // Switch/case statement
    Async(Vec<Stmt>),             // Async block
    Invalid(String),             // Invalid statement (error recovery)
    NativeFunction {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },                            // Native function definition
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
    Mod,
    Concat,         // String concatenation
    In,             // Membership check
    Contains,       // Check presence in list/dict
    FieldAccess,    // Access field
    InstanceOf,     // Type check
    BitAnd,         // Binary AND
    BitOr,          // Binary OR
    BitXor,         // Binary XOR
    BitShiftLeft,   // Left shift
    BitShiftRight,  // Right shift
    BitNot,         // Bitwise NOT
    Range,          // Range operator (exclusive)
    RangeInclusive, // Range operator (inclusive)
    NullCoalesce,   // Null coalescing operator (??)
    ArrowFn,        // Arrow function operator (=>)
    Is,             // Type checking operator
    Pipeline,       // Function pipeline operator (|>)
    SetUnion,       // Set union operator (∪)
    SetIntersection,// Set intersection operator (∩)
    SetDifference,  // Set difference operator (−)
}

/// Opérateurs unaires
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,        // Numeric negation
    Not,        // Logical not
    Ref,        // Reference operator (&)
    Deref,      // Dereference operator (*)
}