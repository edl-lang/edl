// core/src/runtime.rs

use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Function(Function),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
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
                    .ok_or_else(|| RuntimeError::Message(format!("Undefined variable '{name}'")))
            }
            Expr::Assign { name, expr } => {
                let val = self.eval_expr(expr)?;
                if self.env.assign(name, val.clone()) {
                    Ok(val)
                } else {
                    Err(RuntimeError::Message(format!("Undefined variable '{name}'")))
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
        }
    }
}

fn eval_body(body: &[Stmt], interp: &mut Interpreter) -> Result<Value, RuntimeError> {
    let mut result = Value::Null;
    for stmt in body {
        result = interp.eval_stmt(stmt)?;
    }
    Ok(result)
}

fn eval_binary(op: BinOp, l: Value, r: Value) -> Result<Value, RuntimeError> {
    use BinOp::*;
    Ok(match op {
        Add => Value::Number(get_num(&l) + get_num(&r)),
        Sub => Value::Number(get_num(&l) - get_num(&r)),
        Mul => Value::Number(get_num(&l) * get_num(&r)),
        Div => Value::Number(get_num(&l) / get_num(&r)),
        Eq => Value::Bool(l == r),
        Neq => Value::Bool(l != r),
        Lt => Value::Bool(get_num(&l) < get_num(&r)),
        Lte => Value::Bool(get_num(&l) <= get_num(&r)),
        Gt => Value::Bool(get_num(&l) > get_num(&r)),
        Gte => Value::Bool(get_num(&l) >= get_num(&r)),
        And => Value::Bool(is_truthy(&l) && is_truthy(&r)),
        Or => Value::Bool(is_truthy(&l) || is_truthy(&r)),
    })
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Bool(b) => *b,
        Value::Null => false,
        Value::Number(n) => *n != 0.0,
        Value::String(s) => !s.is_empty(),
        _ => true,
    }
}

fn get_num(val: &Value) -> f64 {
    match val {
        Value::Number(n) => *n,
        Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        Value::Null => 0.0,
        _ => 0.0,
    }
}