use crate::ast::{Expr, Literal, Op, Stmt};

pub struct LLVMGenerator {
    globals: String,
    main_body: String,
    temp_counter: usize,
    active_params: Vec<String>,
    struct_defs: std::collections::HashMap<String, Vec<String>>,
    var_types: std::collections::HashMap<String, String>,
    scopes: Vec<std::collections::HashSet<String>>,
    moved_vars: std::collections::HashSet<String>,
    allocated_vars: std::collections::HashSet<String>,
}

impl LLVMGenerator {
    pub fn new() -> Self {
        LLVMGenerator { 
            globals: String::new(), 
            main_body: String::new(), 
            temp_counter: 0, 
            active_params: Vec::new(),
            struct_defs: std::collections::HashMap::new(),
            var_types: std::collections::HashMap::new(),
            scopes: vec![std::collections::HashSet::new()],
            moved_vars: std::collections::HashSet::new(),
            allocated_vars: std::collections::HashSet::new(),
        }
    }

    fn fresh_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("%t{}", self.temp_counter)
    }

    
    fn resolve_var_ptr(&self, name: &str) -> String {
        if self.active_params.contains(&name.to_string()) {
            format!("{}.addr", name)
        } else {
            name.to_string()
        }
    }

    fn gen_auto_drop(&mut self) -> String {
        let mut out = String::new();
        let vars_to_drop: Vec<String> = self.scopes.last().unwrap_or(&std::collections::HashSet::new())
            .iter()
            .filter(|v| !self.moved_vars.contains(*v))
            .cloned()
            .collect();
            
        for var in vars_to_drop {
            let ptr = self.resolve_var_ptr(&var);
            let temp = self.fresh_temp();
            out.push_str(&format!("  {} = load i64, i64* %{}\n", temp, ptr));
            let cast_ptr = self.fresh_temp();
            out.push_str(&format!("  {} = inttoptr i64 {} to i8*\n", cast_ptr, temp));
            out.push_str(&format!("  call void @free(i8* {})\n", cast_ptr));
        }
        out
    }

    pub fn generate(&mut self, program: &[Stmt]) -> String {
        self.globals.push_str("; Target Module: Krait Native Engine\n");
        self.globals.push_str("declare i32 @printf(i8*, ...)\n");
        self.globals.push_str("declare i8* @malloc(i64)\n");
        self.globals.push_str("declare void @free(i8*)\n");
        self.globals.push_str("@.int_fmt = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\"\n\n");

        for stmt in program {
            if let Stmt::StructDef { name, fields } = stmt {
                let field_names = fields.iter().map(|(n, _)| n.clone()).collect();
                self.struct_defs.insert(name.clone(), field_names);
            }
        }

        self.main_body.push_str("define i32 @main() {\nentry:\n");
        self.scopes.push(std::collections::HashSet::new());
        self.allocated_vars.clear();

        for stmt in program {
            match stmt {
                Stmt::FunctionDef { name, params, body } => {
                    self.active_params = params.clone();
                    self.allocated_vars.clear();
                    let params_decl = params.iter().map(|p| format!("i64 %{}", p)).collect::<Vec<_>>().join(", ");
                    self.globals.push_str(&format!("define i64 @{}({}) {{\nentry:\n", name, params_decl));
                    
                    let mut body_ir = String::new();
                    for param in params {
                        body_ir.push_str(&format!("  %{}.addr = alloca i64\n", param));
                        body_ir.push_str(&format!("  store i64 %{}, i64* %{}.addr\n", param, param));
                    }

                    self.scopes.push(std::collections::HashSet::new());
                    for s in body { self.gen_stmt(s, &mut body_ir); }
                    let drops = self.gen_auto_drop();
                    body_ir.push_str(&drops);
                    self.scopes.pop();

                    if !body_ir.contains("ret i64") { body_ir.push_str("  ret i64 0\n"); }
                    
                    self.globals.push_str(&body_ir);
                    self.globals.push_str("}\n\n");
                    self.active_params.clear();
                }
                Stmt::ExternDecl { name, params } => {
                    let params_decl = params.iter().map(|_| "i64").collect::<Vec<_>>().join(", ");
                    self.globals.push_str(&format!("declare i64 @{}({})\n", name, params_decl));
                }
                Stmt::Import(_) => {}
                _ => {
                    let mut ir = String::new();
                    self.gen_stmt(stmt, &mut ir);
                    self.main_body.push_str(&ir);
                }
            }
        }
        let drops = self.gen_auto_drop();
        self.main_body.push_str(&drops);
        self.scopes.pop();
        self.main_body.push_str("  ret i32 0\n}\n");
        format!("{}{}", self.globals, self.main_body)
    }

    fn gen_stmt(&mut self, stmt: &Stmt, out: &mut String) {
        match stmt {
            Stmt::VariableDecl { name, value } => {
                if let Expr::New(s_name) = value {
                    self.var_types.insert(name.clone(), s_name.clone());
                    if let Some(scope) = self.scopes.last_mut() { scope.insert(name.clone()); }
                } else if let Expr::Variable(v_name) = value {
                    if let Some(t) = self.var_types.get(v_name).cloned() {
                        self.var_types.insert(name.clone(), t);
                        if let Some(scope) = self.scopes.last_mut() { scope.insert(name.clone()); }
                        self.moved_vars.insert(v_name.clone());
                    }
                }
                let val_reg = self.gen_expr(value, out);
                let ptr = self.resolve_var_ptr(name);
                if !self.allocated_vars.contains(&ptr) {
                    self.allocated_vars.insert(ptr.clone());
                    out.push_str(&format!("  %{} = alloca i64\n", ptr));
                }
                out.push_str(&format!("  store i64 {}, i64* %{}\n", val_reg, ptr));
            }
            Stmt::FieldAssignment { object, field, value } => {
                let obj_reg = self.gen_expr(object, out);
                let struct_name = if let Expr::Variable(v) = object {
                    self.var_types.get(v).cloned().unwrap_or_default()
                } else {
                    String::new()
                };
                let offset = self.struct_defs.get(&struct_name)
                    .and_then(|fields| fields.iter().position(|f| f == field))
                    .unwrap_or(0) * 8;
                let ptr_reg = self.fresh_temp();
                out.push_str(&format!("  {} = inttoptr i64 {} to i64*\n", ptr_reg, obj_reg));
                let field_ptr = self.fresh_temp();
                out.push_str(&format!("  {} = getelementptr inbounds i64, i64* {}, i32 {}\n", field_ptr, ptr_reg, offset / 8));
                let val_reg = self.gen_expr(value, out);
                out.push_str(&format!("  store i64 {}, i64* {}\n", val_reg, field_ptr));
            }
            Stmt::Show(expr) => {
                let reg = self.gen_expr(expr, out);
                let fmt_ptr = "getelementptr inbounds ([6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0)";
                out.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}, i64 {})\n", fmt_ptr, reg));
            }
            Stmt::Return(Some(expr)) => {
                let reg = self.gen_expr(expr, out);
                out.push_str(&format!("  ret i64 {}\n", reg));
            }
            Stmt::When { cond, then_branch } => {
                let cond_reg = self.gen_expr(cond, out);
                let cond_bool = self.fresh_temp();
                out.push_str(&format!("  {} = icmp ne i64 {}, 0\n", cond_bool, cond_reg));
                
                let then_label = self.fresh_temp().replace("%", "then");
                let end_label = self.fresh_temp().replace("%", "end");
                
                out.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_bool, then_label, end_label));
                out.push_str(&format!("{}:\n", then_label));
                
                self.scopes.push(std::collections::HashSet::new());
                for s in then_branch {
                    self.gen_stmt(s, out);
                }
                let drops = self.gen_auto_drop();
                out.push_str(&drops);
                self.scopes.pop();

                out.push_str(&format!("  br label %{}\n", end_label));
                out.push_str(&format!("{}:\n", end_label));
            }
            Stmt::Repeat { count, body } => {
                let count_reg = self.gen_expr(count, out);
                let loop_counter = self.fresh_temp();
                let loop_counter_clean = loop_counter.replace("%", "");
                
                out.push_str(&format!("  %{} = alloca i64\n", loop_counter_clean));
                out.push_str(&format!("  store i64 0, i64* %{}\n", loop_counter_clean));
                
                let cond_label = self.fresh_temp().replace("%", "loop_cond");
                let body_label = self.fresh_temp().replace("%", "loop_body");
                let end_label = self.fresh_temp().replace("%", "loop_end");
                
                out.push_str(&format!("  br label %{}\n", cond_label));
                out.push_str(&format!("{}:\n", cond_label));
                
                let curr_counter = self.fresh_temp();
                out.push_str(&format!("  {} = load i64, i64* %{}\n", curr_counter, loop_counter_clean));
                let cmp_res = self.fresh_temp();
                out.push_str(&format!("  {} = icmp slt i64 {}, {}\n", cmp_res, curr_counter, count_reg));
                
                out.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cmp_res, body_label, end_label));
                out.push_str(&format!("{}:\n", body_label));
                
                self.scopes.push(std::collections::HashSet::new());
                for s in body {
                    self.gen_stmt(s, out);
                }
                let drops = self.gen_auto_drop();
                out.push_str(&drops);
                self.scopes.pop();
                
                let loaded_counter = self.fresh_temp();
                out.push_str(&format!("  {} = load i64, i64* %{}\n", loaded_counter, loop_counter_clean));
                let next_counter = self.fresh_temp();
                out.push_str(&format!("  {} = add i64 {}, 1\n", next_counter, loaded_counter));
                out.push_str(&format!("  store i64 {}, i64* %{}\n", next_counter, loop_counter_clean));
                
                out.push_str(&format!("  br label %{}\n", cond_label));
                out.push_str(&format!("{}:\n", end_label));
            }
            Stmt::Expr(expr) => {
                self.gen_expr(expr, out);
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
                out.push_str(&format!("  {} = load i64, i64* %{}\n", temp, ptr));
                temp
            }
            Expr::New(name) => {
                let size = self.struct_defs.get(name).map(|f| f.len()).unwrap_or(0) * 8;
                let ptr = self.fresh_temp();
                out.push_str(&format!("  {} = call i8* @malloc(i64 {})\n", ptr, size));
                let cast_ptr = self.fresh_temp();
                out.push_str(&format!("  {} = ptrtoint i8* {} to i64\n", cast_ptr, ptr));
                cast_ptr
            }
            Expr::FieldAccess { object, field } => {
                let obj_reg = self.gen_expr(object, out);
                let struct_name = if let Expr::Variable(v) = &**object {
                    self.var_types.get(v).cloned().unwrap_or_default()
                } else {
                    String::new()
                };
                
                let offset = self.struct_defs.get(&struct_name)
                    .and_then(|fields| fields.iter().position(|f| f == field))
                    .unwrap_or(0) * 8;
                
                let ptr_reg = self.fresh_temp();
                out.push_str(&format!("  {} = inttoptr i64 {} to i64*\n", ptr_reg, obj_reg));
                let field_ptr = self.fresh_temp();
                out.push_str(&format!("  {} = getelementptr inbounds i64, i64* {}, i32 {}\n", field_ptr, ptr_reg, offset / 8));
                let val_reg = self.fresh_temp();
                out.push_str(&format!("  {} = load i64, i64* {}\n", val_reg, field_ptr));
                val_reg
            }
            Expr::Binary { left, op, right } => {
                let l_reg = self.gen_expr(left, out);
                let r_reg = self.gen_expr(right, out);
                let dest = self.fresh_temp();
                
                if matches!(op, Op::Eq | Op::Gt | Op::Lt) {
                    let op_str = match op { Op::Eq => "eq", Op::Gt => "sgt", Op::Lt => "slt", _ => "" };
                    let cmp_res = self.fresh_temp();
                    out.push_str(&format!("  {} = icmp {} i64 {}, {}\n", cmp_res, op_str, l_reg, r_reg));
                    out.push_str(&format!("  {} = zext i1 {} to i64\n", dest, cmp_res));
                } else {
                    let op_str = match op { Op::Plus => "add", Op::Minus => "sub", Op::Mul => "mul", Op::Div => "sdiv", _ => "" };
                    out.push_str(&format!("  {} = {} i64 {}, {}\n", dest, op_str, l_reg, r_reg));
                }
                dest
            }
            Expr::Call { callee, args } => {
                let arg_regs: Vec<String> = args.iter().map(|a| self.gen_expr(a, out)).collect();
                let arg_strs: Vec<String> = arg_regs.iter().map(|a| format!("i64 {}", a)).collect();
                let dest = self.fresh_temp();
                out.push_str(&format!("  {} = call i64 @{}({})\n", dest, callee, arg_strs.join(", ")));
                dest
            }
            _ => "0".to_string(),
        }
    }
}