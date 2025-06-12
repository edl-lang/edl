// core/src/runtime.rs

use crate::ast::{BinOp, UnOp, Expr, Stmt};
use std::collections::HashMap;

/// Valeurs manipulées à l'exécution dans EDL
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Function(Function),   // Fonction utilisateur ou native
    Type(Type),           // Définition d'un type (struct/classe)
    Null,
    List(Vec<Value>),
    Instance(Instance),   // Instance d'un type
}

/// Représente une fonction utilisateur EDL
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

/// Représente un type utilisateur (struct/classe)
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: String,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Function>,
}

/// Représente une instance d'un type utilisateur
#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub typ: Type,
    pub fields: HashMap<String, Value>,
}

/// Environnement d'exécution (scope de variables)
#[derive(Clone)]
pub struct Environment {
    pub vars: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    /// Crée un nouvel environnement racine
    pub fn new() -> Self {
        Environment { vars: HashMap::new(), parent: None }
    }

    /// Crée un environnement enfant (pour les blocs, fonctions, etc.)
    pub fn with_parent(parent: &Environment) -> Self {
        Environment {
            vars: HashMap::new(),
            parent: Some(Box::new(parent.clone())),
        }
    }

    /// Recherche une variable dans l'environnement (récursif)
    pub fn get(&self, key: &str) -> Option<Value> {
        if let Some(val) = self.vars.get(key) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(key)
        } else {
            None
        }
    }

    /// Définit une variable dans l'environnement courant
    pub fn set(&mut self, key: String, val: Value) {
        self.vars.insert(key, val);
    }

    /// Affecte une variable (dans l'environnement courant ou parent)
    pub fn assign(&mut self, key: &str, val: Value) -> bool {
        if self.vars.contains_key(key) {
            self.vars.insert(key.to_string(), val);
            true
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign(key, val)
        } else {
            false
        }
    }
}

/// Erreurs d'exécution (runtime)
#[derive(Debug)]
pub enum RuntimeError {
    Message(String),
    Return(Value),
}

/// Interpréteur principal EDL
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    /// Crée un nouvel interpréteur avec les fonctions natives de base
    pub fn new() -> Self {
        let mut env = Environment::new();

        // Ajoute la fonction native input()
        env.set("input".to_string(), Value::Function(Function {
            params: vec![],
            body: vec![], // Le corps est vide, mais on va l'intercepter dans eval_expr
        }));

        // Ajoute la fonction to_number()
        env.set("to_number".to_string(), Value::Function(Function {
            params: vec!["x".to_string()],
            body: vec![], // Intercepte dans eval_expr
        }));

        // Ajoute la fonction length()
        env.set("length".to_string(), Value::Function(Function {
            params: vec!["list".to_string()],
            body: vec![],
        }));

        // Ajoute la fonction push()
        env.set("push".to_string(), Value::Function(Function {
            params: vec!["list".to_string(), "item".to_string()],
            body: vec![],
        }));

        // Ajoute la fonction remove()
        env.set("remove".to_string(), Value::Function(Function {
            params: vec!["list".to_string(), "index".to_string()],
            body: vec![],
        }));

        Interpreter { env }
    }

    /// N'utilise pas de nouvel environnement pour le bloc principal
    pub fn eval_block(&mut self, block: &[Stmt]) -> Result<Value, RuntimeError> {
        let mut last = Value::Null;
        for stmt in block {
            last = self.eval_stmt(stmt)?;
        }
        Ok(last)
    }

    /// Évalue une instruction (statement)
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Value, RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),

            Stmt::Let { name, expr } => {
                let val = self.eval_expr(expr)?;
                self.env.set(name.clone(), val);
                Ok(Value::Null)
            }

            Stmt::Function { name, params, body } => {
                let func = Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                self.env.set(name.clone(), Value::Function(func));
                Ok(Value::Null)
            }

            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Null
                };
                Err(RuntimeError::Return(val))
            }

            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expr(condition)?;
                if is_truthy(&cond) {
                    self.eval_block(then_branch)
                } else if let Some(else_br) = else_branch {
                    self.eval_block(else_br)
                } else {
                    Ok(Value::Null)
                }
            }

            Stmt::While { condition, body } => {
                while is_truthy(&self.eval_expr(condition)?) {
                    let _ = self.eval_block(body)?;
                }
                Ok(Value::Null)
            }

            Stmt::For { var, start, end, body } => {
                let start = get_num(&self.eval_expr(start)?) as i64;
                let end = get_num(&self.eval_expr(end)?) as i64;
                for i in start..end {
                    self.env.set(var.clone(), Value::Number(i as f64));
                    let _ = self.eval_block(body)?;
                }
                Ok(Value::Null)
            }

            Stmt::Import(_) => Ok(Value::Null), // À implémenter

            Stmt::Block(stmts) => self.eval_block(stmts),

            Stmt::Break => Err(RuntimeError::Message("Break not implemented".to_string())),
            Stmt::Continue => Err(RuntimeError::Message("Continue not implemented".to_string())),

            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", value_to_string(&val));
                Ok(Value::Null)
            }

            Stmt::PrintArgs(args) => {
                let mut vals = Vec::new();
                for e in args {
                    let val = self.eval_expr(e)?;
                    vals.push(value_to_string(&val));
                }
                println!("{}", vals.join(" "));
                Ok(Value::Null)
            }

            Stmt::Type { name, fields, methods } => {
                // Déclaration d'un type utilisateur
                let mut fields_map = HashMap::new();
                for (field_name, expr) in fields {
                    let val = self.eval_expr(expr)?;
                    fields_map.insert(field_name.clone(), val);
                }

                let mut methods_map = HashMap::new();
                for stmt in methods {
                    if let Stmt::Function { name: fn_name, params, body } = stmt {
                        let func = Function {
                            params: params.clone(),
                            body: body.clone(),
                        };
                        methods_map.insert(fn_name.clone(), func);
                    }
                }

                let type_val = Value::Type(Type {
                    name: name.clone(),
                    fields: fields_map,
                    methods: methods_map,
                });

                self.env.set(name.clone(), type_val);
                Ok(Value::Null)
            }

            _ => Err(RuntimeError::Message(format!("Not yet implemented: {:?}", stmt))),
        }
    }

    /// Évalue une expression (Expr)
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Variable(name) => {
                self.env
                    .get(name)
                    .ok_or_else(|| RuntimeError::Message(format!("Undefined variable '{}'", name)))
            }
            Expr::Assign { name, expr } => {
                let val = self.eval_expr(expr)?;
                if self.env.assign(name, val.clone()) {
                    Ok(val)
                } else {
                    Err(RuntimeError::Message(format!("Undefined variable '{}'", name)))
                }
            }
            Expr::Block(stmts) => self.eval_block(stmts),
            Expr::Unary { op, expr } => {
                let right = self.eval_expr(expr)?;
                match op {
                    UnOp::Neg => Ok(Value::Number(-get_num(&right))),
                    UnOp::Not => Ok(Value::Bool(!is_truthy(&right))),
                }
            }
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                eval_binary(*op, l, r)
            }
            Expr::Call { function, arguments } => {
                let callee = self.eval_expr(function)?;
                let args = arguments
                    .iter()
                    .map(|arg| self.eval_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                match callee {
                    Value::Function(func) => {
                        // Fonctions natives
                        if func.params.is_empty() && func.body.is_empty() && function_is_input(function) {
                            use std::io::{stdin, stdout, Write};
                            let mut s = String::new();
                            stdout().flush().unwrap();
                            stdin().read_line(&mut s).unwrap();
                            return Ok(Value::String(s.trim().to_string()));
                        }
                        if func.params == vec!["x".to_string()] && func.body.is_empty() && function_is_to_number(function) {
                            if let Some(Value::String(s)) = args.get(0) {
                                if let Ok(n) = s.parse::<f64>() {
                                    return Ok(Value::Number(n));
                                } else {
                                    return Err(RuntimeError::Message("Invalid number".to_string()));
                                }
                            } else {
                                return Err(RuntimeError::Message("to_number expects a string".to_string()));
                            }
                        }
                        if func.params == vec!["list".to_string()] && func.body.is_empty() && function_is_length(function) {
                            if let Some(Value::List(list)) = args.get(0) {
                                return Ok(Value::Number(list.len() as f64));
                            } else {
                                return Err(RuntimeError::Message("length expects a list".to_string()));
                            }
                        }
                        if func.params == vec!["list".to_string(), "item".to_string()] && func.body.is_empty() && function_is_push(function) {
                            if let (Some(Value::List(mut list)), Some(item)) = (args.get(0).cloned(), args.get(1).cloned()) {
                                list.push(item);
                                return Ok(Value::List(list));
                            }
                            return Err(RuntimeError::Message("push expects a list and an item".to_string()));
                        }
                        if func.params == vec!["list".to_string(), "index".to_string()] && func.body.is_empty() && function_is_remove(function) {
                            if let (Some(Value::List(mut list)), Some(Value::Number(idx))) = (args.get(0).cloned(), args.get(1).cloned()) {
                                let i = idx as usize;
                                if i < list.len() {
                                    list.remove(i);
                                    return Ok(Value::List(list));
                                } else {
                                    return Err(RuntimeError::Message("remove: index out of bounds".to_string()));
                                }
                            }
                            return Err(RuntimeError::Message("remove expects a list and an index".to_string()));
                        }
                        // Appel de fonction utilisateur ou méthode liée à une instance
                        if func.params.len() != args.len() {
                            return Err(RuntimeError::Message("Argument count mismatch".to_string()));
                        }
                        let previous_env = self.env.clone();
                        let mut local_env = Environment::with_parent(&previous_env);
                        for (param, arg) in func.params.iter().zip(args) {
                            local_env.set(param.clone(), arg);
                        }
                        self.env = local_env;
                        let result = match eval_body(&func.body, self) {
                            Err(RuntimeError::Return(val)) => Ok(val),
                            other => other,
                        };
                        self.env = previous_env;
                        result
                    }
                    _ => Err(RuntimeError::Message("Can only call functions!".to_string())),
                }
            }
            Expr::List(elements) => {
                let vals = elements.iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::List(vals))
            }
            Expr::Instance { type_name, fields } => {
                // Création d'une instance de type utilisateur
                let type_val = self.env.get(type_name)
                    .ok_or_else(|| RuntimeError::Message(format!("Unknown type '{}'", type_name)))?;
                if let Value::Type(typ) = type_val {
                    let mut instance_fields = typ.fields.clone();
                    for (k, v) in fields {
                        instance_fields.insert(k.clone(), self.eval_expr(v)?);
                    }
                    Ok(Value::Instance(Instance { typ: typ.clone(), fields: instance_fields }))
                } else {
                    Err(RuntimeError::Message(format!("'{}' is not a type", type_name)))
                }
            }
            Expr::FieldAccess { object, field } => {
                let obj = self.eval_expr(object)?;
                match obj {
                    Value::Instance(inst) => {
                        // Méthodes utilisateur
                        if let Some(func) = inst.typ.methods.get(field) {
                            // On "bind" self comme premier paramètre
                            let mut bound_func = func.clone();
                            bound_func.params.insert(0, "self".to_string());
                            Ok(Value::Function(bound_func))
                        } else if let Some(val) = inst.fields.get(field) {
                            Ok(val.clone())
                        } else {
                            Err(RuntimeError::Message(format!("Unknown field or method '{}'", field)))
                        }
                    }
                    Value::List(list) => match field.as_str() {
                        "push" => {
                            Ok(Value::Function(Function {
                                params: vec!["item".to_string()],
                                body: vec![], // Intercepté dans eval_expr
                            }))
                        }
                        "remove" => {
                            Ok(Value::Function(Function {
                                params: vec!["index".to_string()],
                                body: vec![],
                            }))
                        }
                        "length" | "len" => Ok(Value::Number(list.len() as f64)),
                        _ => Err(RuntimeError::Message(format!("Unknown list method '{}'", field))),
                    },
                    Value::String(s) => match field.as_str() {
                        "len" | "length" => Ok(Value::Number(s.len() as f64)),
                        "to_upper" => Ok(Value::String(s.to_uppercase())),
                        "to_lower" => Ok(Value::String(s.to_lowercase())),
                        _ => Err(RuntimeError::Message(format!("Unknown string method '{}'", field))),
                    },
                    _ => Err(RuntimeError::Message("Not an instance".to_string())),
                }
            }
            _ => Err(RuntimeError::Message(format!("Not yet implemented: {:?}", expr))),
        }
    }
}

/// Détermine si une valeur est "vraie" (pour les conditions)
fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Bool(b) => *b,
        Value::Null => false,
        Value::Number(n) => *n != 0.0,
        Value::String(s) => !s.is_empty(),
        Value::Function(_) => true,
        Value::Type(_) => true,
        Value::List(vals) => !vals.is_empty(),
        Value::Instance(_) => true,
    }
}

/// Extrait un nombre d'une Value (ou 0.0 sinon)
fn get_num(val: &Value) -> f64 {
    match val {
        Value::Number(n) => *n,
        _ => 0.0,
    }
}

/// Évalue une opération binaire
fn eval_binary(op: BinOp, left: Value, right: Value) -> Result<Value, RuntimeError> {
    use BinOp::*;
    match op {
        Add => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            _ => Err(RuntimeError::Message("Invalid operands for '+'".to_string())),
        },
        Sub => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(RuntimeError::Message("Invalid operands for '-'".to_string())),
        },
        Mul => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(RuntimeError::Message("Invalid operands for '*'".to_string())),
        },
        Div => match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(RuntimeError::Message("Division by zero".to_string()))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(RuntimeError::Message("Invalid operands for '/'".to_string())),
        },
        Eq => Ok(Value::Bool(left == right)),
        Neq => Ok(Value::Bool(left != right)),
        Lt => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
            _ => Err(RuntimeError::Message("Invalid operands for '<'".to_string())),
        },
        Lte => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
            _ => Err(RuntimeError::Message("Invalid operands for '<='".to_string())),
        },
        Gt => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
            _ => Err(RuntimeError::Message("Invalid operands for '>'".to_string())),
        },
        Gte => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(RuntimeError::Message("Invalid operands for '>='".to_string())),
        },
        And => match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            _ => Err(RuntimeError::Message("Invalid operands for 'and'".to_string())),
        },
        Or => match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            _ => Err(RuntimeError::Message("Invalid operands for 'or'".to_string())),
        },
        Pow => Ok(Value::Number(get_num(&left).powf(get_num(&right)))),
        // Pour les autres opérateurs non encore implémentés
        _ => Err(RuntimeError::Message(format!("Not yet implemented: {:?}", op))),
    }
}

/// Évalue le corps d'une fonction utilisateur
fn eval_body(body: &[Stmt], interpreter: &mut Interpreter) -> Result<Value, RuntimeError> {
    let mut last = Value::Null;
    for stmt in body {
        last = interpreter.eval_stmt(stmt)?;
    }
    Ok(last)
}

/// Convertit une Value en String pour l'affichage
fn value_to_string(val: &Value) -> String {
    match val {
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        Value::Function(_) => "<function>".to_string(),
        Value::Type(t) => format!("<type {}>", t.name),
        Value::List(vals) => {
            let items: Vec<String> = vals.iter().map(|v| value_to_string(v)).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Instance(_) => "<instance>".to_string(),
    }
}

/// Helper pour détecter l'appel à input()
#[allow(dead_code)]
fn function_is_input(function: &Expr) -> bool {
    if let Expr::Variable(name) = function {
        name == "input"
    } else {
        false
    }
}

/// Helper pour détecter l'appel à to_number()
#[allow(dead_code)]
fn function_is_to_number(function: &Expr) -> bool {
    if let Expr::Variable(name) = function {
        name == "to_number"
    } else {
        false
    }
}

/// Helper pour détecter l'appel à length()
#[allow(dead_code)]
fn function_is_length(function: &Expr) -> bool {
    if let Expr::Variable(name) = function {
        name == "length"
    } else {
        false
    }
}

/// Helper pour détecter l'appel à push()
#[allow(dead_code)]
fn function_is_push(function: &Expr) -> bool {
    if let Expr::Variable(name) = function {
        name == "push"
    } else {
        false
    }
}

#[allow(dead_code)]
fn function_is_remove(function: &Expr) -> bool {
    if let Expr::Variable(name) = function {
        name == "remove"
    } else {
        false
    }
}