use std::collections::HashMap;
use crate::ast::{Expr, Literal, Op, Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int, Float, Str, Bool,
}

pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, Type>>,
    function_signatures: HashMap<String, Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: vec![HashMap::new()],
            function_signatures: HashMap::new(),
        }
    }

    fn enter_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn exit_scope(&mut self) { self.scopes.pop(); }
    
    fn declare(&mut self, name: &str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    fn resolve(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) { return Some(ty.clone()); }
        }
        None
    }

    pub fn analyze(&mut self, program: &[Stmt]) -> Result<(), String> {
        for stmt in program {
            if let Stmt::FunctionDef { name, .. } = stmt {
                self.function_signatures.insert(name.clone(), Type::Int);
            }
        }

        for stmt in program {
            self.analyze_statement(stmt)?;
        }
        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::FunctionDef { name: _, params, body } => {
                self.enter_scope();
                for param in params {
                    self.declare(param, Type::Int);
                }
                for s in body { self.analyze_statement(s)?; }
                self.exit_scope();
            }
            Stmt::VariableDecl { name, value } => {
                let ty = self.infer_type(value)?;
                self.declare(name, ty);
            }
            Stmt::When { cond, then_branch } => {
                let cond_ty = self.infer_type(cond)?;
                if cond_ty != Type::Bool { return Err("Condition must be boolean.".into()); }
                self.enter_scope();
                for s in then_branch { self.analyze_statement(s)?; }
                self.exit_scope();
            }
            Stmt::Repeat { count, body } => {
                let count_ty = self.infer_type(count)?;
                if count_ty != Type::Int { return Err("Repeat count must be an integer.".into()); }
                self.enter_scope();
                for s in body { self.analyze_statement(s)?; }
                self.exit_scope();
            }
            Stmt::Show(expr) | Stmt::Return(Some(expr)) | Stmt::Expr(expr) => {
                self.infer_type(expr)?;
            }
            Stmt::Return(None) | Stmt::StructDef { .. } => {}
        }
        Ok(())
    }

    fn infer_type(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(_) => Ok(Type::Int),
                Literal::Float(_) => Ok(Type::Float),
                Literal::Str(_) => Ok(Type::Str),
                Literal::Bool(_) => Ok(Type::Bool),
            },
            Expr::Variable(name) => self.resolve(name)
                .ok_or_else(|| format!("Variable '{}' used before declaration.", name)),
            Expr::Binary { left, op, right } => {
                let lt = self.infer_type(left)?;
                let rt = self.infer_type(right)?;
                if lt != rt { return Err(format!("Mismatched types: {:?} vs {:?}", lt, rt)); }
                match op {
                    Op::Eq | Op::Gt | Op::Lt => Ok(Type::Bool),
                    _ => Ok(lt),
                }
            }
            Expr::Call { callee, args } => {
                for arg in args { self.infer_type(arg)?; }
                if !self.function_signatures.contains_key(callee) {
                    return Err(format!("Calling undefined function: '{}'", callee));
                }
                Ok(Type::Int)
            }
        }
    }
}