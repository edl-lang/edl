// core/src/runtime.rs

use crate::ast::{BinOp, UnOp, Expr, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Function(Function),
    Type(Type),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: String,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Function>,
}

#[derive(Clone)]
pub struct Environment {
    pub vars: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { vars: HashMap::new(), parent: None }
    }

    pub fn with_parent(parent: &Environment) -> Self {
        Environment {
            vars: HashMap::new(),
            parent: Some(Box::new(parent.clone())),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        if let Some(val) = self.vars.get(key) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(key)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: String, val: Value) {
        self.vars.insert(key, val);
    }

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

#[derive(Debug)]
pub enum RuntimeError {
    Message(String),
    Return(Value),
}

pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Environment::new() }
    }

    pub fn eval_block(&mut self, block: &[Stmt]) -> Result<Value, RuntimeError> {
        let previous_env = self.env.clone();
        self.env = Environment::with_parent(&previous_env);
        let result = (|| {
            let mut last = Value::Null;
            for stmt in block {
                last = self.eval_stmt(stmt)?;
            }
            Ok(last)
        })();
        self.env = previous_env;
        result
    }

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
                    let previous_env = self.env.clone();
                    self.env = Environment::with_parent(&previous_env);
                    let _ = self.eval_block(body)?;
                    self.env = previous_env;
                }
                Ok(Value::Null)
            }

            Stmt::For { var, start, end, body } => {
                let start = get_num(&self.eval_expr(start)?) as i64;
                let end = get_num(&self.eval_expr(end)?) as i64;
                for i in start..end {
                    let previous_env = self.env.clone();
                    self.env = Environment::with_parent(&previous_env);
                    self.env.set(var.clone(), Value::Number(i as f64));
                    let _ = self.eval_block(body)?;
                    self.env = previous_env;
                }
                Ok(Value::Null)
            }

            Stmt::Import(_) => Ok(Value::Null),

            Stmt::Block(stmts) => self.eval_block(stmts),

            Stmt::Break => Err(RuntimeError::Message("Break not implemented".to_string())),
            Stmt::Continue => Err(RuntimeError::Message("Continue not implemented".to_string())),

            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", value_to_string(&val));
                Ok(Value::Null)
            }

            Stmt::Type { name, fields, methods } => {
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
            _ => Err(RuntimeError::Message(format!("Not yet implemented: {:?}", expr))),
        }
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Bool(b) => *b,
        Value::Null => false,
        Value::Number(n) => *n != 0.0,
        Value::String(s) => !s.is_empty(),
        Value::Function(_) => true,
        Value::Type(_) => true,
    }
}

fn get_num(val: &Value) -> f64 {
    match val {
        Value::Number(n) => *n,
        _ => 0.0,
    }
}

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
            _ => Err(RuntimeError::Message("Invalid operands for '&&'".to_string())),
        },
        Or => match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            _ => Err(RuntimeError::Message("Invalid operands for '||'".to_string())),
        },
    }
}

fn eval_body(body: &[Stmt], interpreter: &mut Interpreter) -> Result<Value, RuntimeError> {
    let mut last = Value::Null;
    for stmt in body {
        last = interpreter.eval_stmt(stmt)?;
    }
    Ok(last)
}

fn value_to_string(val: &Value) -> String {
    match val {
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        Value::Function(_) => "<function>".to_string(),
        Value::Type(t) => format!("<type {}>", t.name),
    }
}