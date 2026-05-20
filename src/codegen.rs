use crate::ast::{Expr, Literal, Op, Stmt};

pub struct LLVMGenerator {
    globals: String,
    main_body: String,
    temp_counter: usize,
    active_params: Vec<String>, // Phase 2: Parameter Stack Tracking
}

impl LLVMGenerator {
    pub fn new() -> Self {
        LLVMGenerator { globals: String::new(), main_body: String::new(), temp_counter: 0, active_params: Vec::new() }
    }

    fn fresh_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("%t{}", self.temp_counter)
    }

    // Resolves parameter registers to stack pointers if they are in local scope
    fn resolve_var_ptr(&self, name: &str) -> String {
        if self.active_params.contains(&name.to_string()) {
            format!("{}.addr", name)
        } else {
            name.to_string()
        }
    }

    pub fn generate(&mut self, program: &[Stmt]) -> String {
        self.globals.push_str("; Target Module: Krait Native Engine\n");
        self.globals.push_str("declare i32 @printf(i8*, ...)\n");
        self.globals.push_str("@.int_fmt = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n\n");

        self.main_body.push_str("define i32 @main() {\nentry:\n");

        for stmt in program {
            match stmt {
                Stmt::FunctionDef { name, params, body } => {
                    self.active_params = params.clone();
                    let params_decl = params.iter().map(|p| format!("i32 %{}", p)).collect::<Vec<_>>().join(", ");
                    self.globals.push_str(&format!("define i32 @{}({}) {{\nentry:\n", name, params_decl));
                    
                    let mut body_ir = String::new();
                    // Map parameters directly to stack-allocated variables
                    for param in params {
                        body_ir.push_str(&format!("  %{}.addr = alloca i32\n", param));
                        body_ir.push_str(&format!("  store i32 %{}, i32* %{}.addr\n", param, param));
                    }

                    for s in body { self.gen_stmt(s, &mut body_ir); }
                    if !body_ir.contains("ret i32") { body_ir.push_str("  ret i32 0\n"); }
                    
                    self.globals.push_str(&body_ir);
                    self.globals.push_str("}\n\n");
                    self.active_params.clear();
                }
                _ => {
                    let mut ir = String::new();
                    self.gen_stmt(stmt, &mut ir);
                    self.main_body.push_str(&ir);
                }
            }
        }
        self.main_body.push_str("  ret i32 0\n}\n");
        format!("{}{}", self.globals, self.main_body)
    }

    fn gen_stmt(&mut self, stmt: &Stmt, out: &mut String) {
        match stmt {
            Stmt::VariableDecl { name, value } => {
                let val_reg = self.gen_expr(value, out);
                let ptr = self.resolve_var_ptr(name);
                out.push_str(&format!("  %{} = alloca i32\n", ptr));
                out.push_str(&format!("  store i32 {}, i32* %{}\n", val_reg, ptr));
            }
            Stmt::Show(expr) => {
                let reg = self.gen_expr(expr, out);
                let fmt_ptr = "getelementptr inbounds ([4 x i8], [4 x i8]* @.int_fmt, i32 0, i32 0)";
                out.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}, i32 {})\n", fmt_ptr, reg));
            }
            Stmt::Return(Some(expr)) => {
                let reg = self.gen_expr(expr, out);
                out.push_str(&format!("  ret i32 {}\n", reg));
            }
            Stmt::When { cond, then_branch } => {
                let cond_reg = self.gen_expr(cond, out);
                let cond_bool = self.fresh_temp();
                out.push_str(&format!("  {} = icmp ne i32 {}, 0\n", cond_bool, cond_reg));
                
                let then_label = self.fresh_temp().replace("%", "then");
                let end_label = self.fresh_temp().replace("%", "end");
                
                out.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_bool, then_label, end_label));
                out.push_str(&format!("{}:\n", then_label));
                for s in then_branch { self.gen_stmt(s, out); }
                out.push_str(&format!("  br label %{}\n", end_label));
                out.push_str(&format!("{}:\n", end_label));
            }
            _ => {}
        }
    }

    fn gen_expr(&mut self, expr: &Expr, out: &mut String) -> String {
        match expr {
            Expr::Literal(Literal::Int(v)) => v.to_string(),
            Expr::Literal(Literal::Bool(v)) => if *v { "1".into() } else { "0".into() },
            Expr::Variable(name) => {
                let temp = self.fresh_temp();
                let ptr = self.resolve_var_ptr(name);
                out.push_str(&format!("  {} = load i32, i32* %{}\n", temp, ptr));
                temp
            }
            Expr::Binary { left, op, right } => {
                let l_reg = self.gen_expr(left, out);
                let r_reg = self.gen_expr(right, out);
                let dest = self.fresh_temp();
                
                if matches!(op, Op::Eq | Op::Gt | Op::Lt) {
                    let op_str = match op { Op::Eq => "eq", Op::Gt => "sgt", Op::Lt => "slt", _ => "" };
                    let cmp_res = self.fresh_temp();
                    out.push_str(&format!("  {} = icmp {} i32 {}, {}\n", cmp_res, op_str, l_reg, r_reg));
                    out.push_str(&format!("  {} = zext i1 {} to i32\n", dest, cmp_res));
                } else {
                    let op_str = match op { Op::Plus => "add", Op::Minus => "sub", Op::Mul => "mul", Op::Div => "sdiv", _ => "" };
                    out.push_str(&format!("  {} = {} i32 {}, {}\n", dest, op_str, l_reg, r_reg));
                }
                dest
            }
            Expr::Call { callee, args } => {
                let arg_regs: Vec<String> = args.iter().map(|a| self.gen_expr(a, out)).collect();
                let arg_strs: Vec<String> = arg_regs.iter().map(|a| format!("i32 {}", a)).collect();
                let dest = self.fresh_temp();
                out.push_str(&format!("  {} = call i32 @{}({})\n", dest, callee, arg_strs.join(", ")));
                dest
            }
            _ => "0".to_string(),
        }
    }
}