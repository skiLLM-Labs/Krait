use crate::ast::{Expr, Literal, Op, Stmt};

pub struct Formatter {
    indent_level: usize,
}

impl Formatter {
    pub fn new() -> Self {
        Formatter { indent_level: 0 }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn format(&mut self, program: &[Stmt]) -> String {
        let mut out = String::new();
        for stmt in program {
            out.push_str(&self.format_statement(stmt));
        }
        out
    }

    fn format_statement(&mut self, stmt: &Stmt) -> String {
        match stmt {
            Stmt::FunctionDef { name, params, body } => {
                let params_str = params.join(", ");
                let mut out = format!("{}make {}({})\n", self.indent(), name, params_str);
                self.indent_level += 1;
                for s in body {
                    out.push_str(&self.format_statement(s));
                }
                self.indent_level -= 1;
                out.push('\n');
                out
            }
            Stmt::ExternDecl { name, params } => {
                let params_str = params.join(", ");
                format!("{}extern make {}({})\n", self.indent(), name, params_str)
            }
            Stmt::StructDef { name, fields } => {
                let mut out = format!("{}make {}\n", self.indent(), name);
                self.indent_level += 1;
                for (field, val) in fields {
                    out.push_str(&format!("{}{} = {}\n", self.indent(), field, self.format_expr(val)));
                }
                self.indent_level -= 1;
                out.push('\n');
                out
            }
            Stmt::Import(name) => format!("import {}\n", name),
            Stmt::VariableDecl { name, value } => {
                format!("{}set {} = {}\n", self.indent(), name, self.format_expr(value))
            }
            Stmt::FieldAssignment { object, field, value } => {
                format!("{}set {}.{} = {}\n", self.indent(), self.format_expr(object), field, self.format_expr(value))
            }
            Stmt::When { cond, then_branch } => {
                let mut out = format!("{}when {}\n", self.indent(), self.format_expr(cond));
                self.indent_level += 1;
                for s in then_branch {
                    out.push_str(&self.format_statement(s));
                }
                self.indent_level -= 1;
                out
            }
            Stmt::Repeat { count, body } => {
                let mut out = format!("{}repeat {} times\n", self.indent(), self.format_expr(count));
                self.indent_level += 1;
                for s in body {
                    out.push_str(&self.format_statement(s));
                }
                self.indent_level -= 1;
                out
            }
            Stmt::Show(expr) => {
                format!("{}show {}\n", self.indent(), self.format_expr(expr))
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    format!("{}return {}\n", self.indent(), self.format_expr(expr))
                } else {
                    format!("{}return\n", self.indent())
                }
            }
            Stmt::Expr(expr) => {
                format!("{}{}\n", self.indent(), self.format_expr(expr))
            }
        }
    }

    fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(v) => v.to_string(),
                Literal::Float(v) => v.to_string(),
                Literal::Str(v) => format!("\"{}\"", v),
                Literal::Bool(v) => v.to_string(),
            },
            Expr::Variable(name) => name.clone(),
            Expr::New(name) => format!("new {}", name),
            Expr::FieldAccess { object, field } => {
                format!("{}.{}", self.format_expr(object), field)
            }
            Expr::Binary { left, op, right } => {
                let op_str = match op {
                    Op::Plus => "+",
                    Op::Minus => "-",
                    Op::Mul => "*",
                    Op::Div => "/",
                    Op::Eq => "==",
                    Op::Gt => ">",
                    Op::Lt => "<",
                };
                format!("{} {} {}", self.format_expr(left), op_str, self.format_expr(right))
            }
            Expr::Call { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!("{}({})", callee, args_str.join(", "))
            }
        }
    }
}