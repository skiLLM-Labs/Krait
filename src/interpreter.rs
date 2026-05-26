use std::collections::HashMap;
use crate::ast::{Expr, Literal, Op, Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Void,
}

pub struct Interpreter {
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    pub env: HashMap<String, Value>, 
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            functions: HashMap::new(),
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &[Stmt]) -> Result<Option<Value>, String> {
        
        for stmt in program {
            if let Stmt::FunctionDef { name, params, body } = stmt {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
            }
        }

        let mut last_val = None;
        
        
        for stmt in program {
            if !matches!(stmt, Stmt::FunctionDef { .. } | Stmt::StructDef { .. }) {
                
                
                let mut local_env = self.env.clone();
                last_val = self.execute_stmt(stmt, &mut local_env)?;
            }
        }
        Ok(last_val)
    }

    fn execute_stmt(&mut self, stmt: &Stmt, env: &mut HashMap<String, Value>) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::FunctionDef { .. } | Stmt::StructDef { .. } | Stmt::ExternDecl { .. } => Ok(None),
            Stmt::Import(_) => Ok(None),
            Stmt::VariableDecl { name, value } => {
                let val = self.eval_expr(value, env)?;
                env.insert(name.clone(), val.clone());
                self.env.insert(name.clone(), val); 
                Ok(None)
            }
            Stmt::FieldAssignment { .. } => {
                Err("Field assignment not supported in interpreter".into())
            }
            Stmt::When { cond, then_branch } => {
                let cond_val = self.eval_expr(cond, env)?;
                if let Value::Bool(true) = cond_val {
                    let mut local_env = env.clone();
                    for s in then_branch {
                        if let Some(ret) = self.execute_stmt(s, &mut local_env)? {
                            for (key, val) in &local_env {
                                if env.contains_key(key) {
                                    env.insert(key.clone(), val.clone());
                                }
                            }
                            return Ok(Some(ret));
                        }
                    }
                    for (key, val) in &local_env {
                        if env.contains_key(key) {
                            env.insert(key.clone(), val.clone());
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Repeat { count, body } => {
                let count_val = self.eval_expr(count, env)?;
                if let Value::Int(n) = count_val {
                    for _ in 0..n {
                        let mut local_env = env.clone();
                        for s in body {
                            if let Some(ret) = self.execute_stmt(s, &mut local_env)? {
                                for (key, val) in &local_env {
                                    if env.contains_key(key) {
                                        env.insert(key.clone(), val.clone());
                                    }
                                }
                                return Ok(Some(ret));
                            }
                        }
                        for (key, val) in &local_env {
                            if env.contains_key(key) {
                                env.insert(key.clone(), val.clone());
                            }
                        }
                    }
                } else {
                    return Err("Repeat limit must evaluate to an integer type.".to_string());
                }
                Ok(None)
            }
            Stmt::Show(expr) => {
                let val = self.eval_expr(expr, env)?;
                match val {
                    Value::Int(v) => println!("{}", v),
                    Value::Float(v) => println!("{}", v),
                    Value::Str(v) => println!("{}", v),
                    Value::Bool(v) => println!("{}", v),
                    Value::Void => println!("void"),
                }
                Ok(None)
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let val = self.eval_expr(expr, env)?;
                    Ok(Some(val))
                } else {
                    Ok(Some(Value::Void))
                }
            }
            Stmt::Expr(expr) => {
                let val = self.eval_expr(expr, env)?;
                Ok(Some(val)) 
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: &HashMap<String, Value>) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(v) => Ok(Value::Int(*v)),
                Literal::Float(v) => Ok(Value::Float(*v)),
                Literal::Str(v) => Ok(Value::Str(v.clone())),
                Literal::Bool(v) => Ok(Value::Bool(*v)),
            },
            Expr::Variable(name) => {
                
                if let Some(val) = env.get(name).or_else(|| self.env.get(name)) {
                    Ok(val.clone())
                } else {
                    Err(format!("Undefined identifier: '{}'", name))
                }
            }
            Expr::New(_name) => Err("Heap allocation not supported in interpreter".into()),
            Expr::FieldAccess { object: _, field: _ } => {
                Err("Field access not supported in interpreter yet".into())
            }
            Expr::Binary { left, op, right } => {
                let l_val = self.eval_expr(left, env)?;
                let r_val = self.eval_expr(right, env)?;
                match (l_val, r_val) {
                    (Value::Int(l), Value::Int(r)) => match op {
                        Op::Plus => Ok(Value::Int(l + r)),
                        Op::Minus => Ok(Value::Int(l - r)),
                        Op::Mul => Ok(Value::Int(l * r)),
                        Op::Div => {
                            if r == 0 { return Err("Division by zero error.".to_string()); }
                            Ok(Value::Int(l / r))
                        }
                        Op::Eq => Ok(Value::Bool(l == r)),
                        Op::Gt => Ok(Value::Bool(l > r)),
                        Op::Lt => Ok(Value::Bool(l < r)),
                    },
                    (Value::Float(l), Value::Float(r)) => match op {
                        Op::Plus => Ok(Value::Float(l + r)),
                        Op::Minus => Ok(Value::Float(l - r)),
                        Op::Mul => Ok(Value::Float(l * r)),
                        Op::Div => Ok(Value::Float(l / r)),
                        Op::Eq => Ok(Value::Bool(l == r)),
                        Op::Gt => Ok(Value::Bool(l > r)),
                        Op::Lt => Ok(Value::Bool(l < r)),
                    },
                    (Value::Str(l), Value::Str(r)) => match op {
                        Op::Plus => Ok(Value::Str(format!("{}{}", l, r))),
                        Op::Eq => Ok(Value::Bool(l == r)),
                        _ => Err("Invalid operator mapping applied to string type parameters.".to_string()),
                    },
                    (Value::Bool(l), Value::Bool(r)) => match op {
                        Op::Eq => Ok(Value::Bool(l == r)),
                        _ => Err("Invalid operator mapping applied to boolean type parameters.".to_string()),
                    },
                    _ => Err("Type mismatch on target operation.".to_string()),
                }
            }
            Expr::Call { callee, args } => {
                let (params, body) = self
                    .functions
                    .get(callee)
                    .cloned()
                    .ok_or_else(|| format!("Undefined function call: '{}'", callee))?;

                if params.len() != args.len() {
                    return Err(format!("Arg length mismatch: Expected {} but received {}.", params.len(), args.len()));
                }

                let mut call_env = HashMap::new();
                for (param, arg) in params.iter().zip(args.iter()) {
                    let arg_val = self.eval_expr(arg, env)?;
                    call_env.insert(param.clone(), arg_val);
                }

                for s in &body {
                    if let Some(ret_val) = self.execute_stmt(s, &mut call_env)? {
                        return Ok(ret_val);
                    }
                }
                Ok(Value::Void)
            }
        }
    }
}