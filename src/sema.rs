use std::collections::HashMap;
use crate::ast::{Expr, Literal, Op, Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int, Float, Str, Bool, Struct(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarState {
    Valid(Type),
    Moved(Type),
}

pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, VarState>>,
    function_signatures: HashMap<String, Type>,
    struct_definitions: HashMap<String, HashMap<String, Type>>,
    pub auto_drops: HashMap<usize, Vec<String>>, 
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: vec![HashMap::new()],
            function_signatures: HashMap::new(),
            struct_definitions: HashMap::new(),
            auto_drops: HashMap::new(),
        }
    }

    fn enter_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn exit_scope(&mut self) { self.scopes.pop(); }
    
    pub fn declare(&mut self, name: &str, ty: Type) {
        if matches!(ty, Type::Struct(_)) {
            let depth = self.scopes.len() - 1;
            self.auto_drops.entry(depth).or_default().push(name.to_string());
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), VarState::Valid(ty));
        }
    }

    pub fn resolve(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(state) = scope.get(name) {
                match state {
                    VarState::Valid(ty) => return Some(ty.clone()),
                    VarState::Moved(_) => return None, 
                }
            }
        }
        None
    }

    pub fn check_moved(&self, name: &str) -> Result<(), String> {
        for scope in self.scopes.iter().rev() {
            if let Some(state) = scope.get(name) {
                if let VarState::Moved(_) = state {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT OWNERSHiP ERROR]\n\
                         ================================================================================\n\
                         Variable: '{name}'\n\
                         Issue   : The variable '{name}' was moved and is no longer valid in this scope.\n\n\
                         Why this happens:\n\
                         Krait employs a high-performance Rust-like Ownership Model with automatic memory deallocation.\n\
                         When you assign a struct/heap variable to another variable (e.g. `set a = b`), the ownership\n\
                         of the underlying memory is transferred (moved) to the new variable. The original variable\n\
                         is invalidated to prevent double-free and use-after-free vulnerabilities at runtime.\n\n\
                         How to resolve:\n\
                         1. Avoid using '{name}' after it has been reassigned/moved.\n\
                         2. If you need to access '{name}' later, duplicate the data or reorganize your code to assign\n\
                            the new variable only when you are done using '{name}'.\n\n\
                         Memory Safety Guaranteed: Zero-cost abstraction without a garbage collector.\n\
                         ================================================================================\n"
                    ));
                }
            }
        }
        Ok(())
    }
    
    pub fn move_var(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(state) = scope.get_mut(name) {
                if let VarState::Valid(ty) = state {
                    *state = VarState::Moved(ty.clone());
                    break;
                }
            }
        }
    }

    pub fn analyze(&mut self, program: &[Stmt]) -> Result<(), String> {
        for stmt in program {
            match stmt {
                Stmt::FunctionDef { name, .. } | Stmt::ExternDecl { name, .. } => {
                    self.function_signatures.insert(name.clone(), Type::Int);
                }
                Stmt::StructDef { name, fields } => {
                    let mut field_types = HashMap::new();
                    for (f_name, f_expr) in fields {
                        let ty = self.infer_type(f_expr)?;
                        field_types.insert(f_name.clone(), ty);
                    }
                    self.struct_definitions.insert(name.clone(), field_types);
                }
                Stmt::Import(_) => {}
                _ => {}
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
                
                if let Expr::Variable(v) = value {
                    if matches!(ty, Type::Struct(_)) {
                        self.move_var(v);
                    }
                }
                
                self.declare(name, ty);
            }
            Stmt::FieldAssignment { object, field, value } => {
                let obj_ty = self.infer_type(object)?;
                let object_str = format!("{:?}", object);
                if let Type::Struct(struct_name) = obj_ty {
                    if let Some(def) = self.struct_definitions.get(&struct_name) {
                        if let Some(field_ty) = def.get(field) {
                            let val_ty = self.infer_type(value)?;
                            if val_ty != *field_ty {
                                return Err(format!(
                                    "\n================================================================================\n\
                                     [KRAiT TYPE MISMATCH ERROR]\n\
                                     ================================================================================\n\
                                     Field   : '{object_str}.{field}'\n\
                                     Expected: {field_ty:?}\n\
                                     Got     : {val_ty:?}\n\
                                     Issue   : Mismatched types in field assignment.\n\n\
                                     Why this happens:\n\
                                     The value being assigned to the field does not match the field's declared type.\n\
                                     Krait enforces strict type checking on struct fields to ensure memory safety\n\
                                     and predictable layouts.\n\n\
                                     How to resolve:\n\
                                     1. Ensure that the assigned expression evaluates to a value of type '{field_ty:?}'.\n\
                                     ================================================================================\n"
                                ));
                            }
                        } else {
                            return Err(format!(
                                "\n================================================================================\n\
                                 [KRAiT STRUCT FiELD ERROR]\n\
                                 ================================================================================\n\
                                 Struct  : '{struct_name}'\n\
                                 Field   : '{field}'\n\
                                 Issue   : Field '{field}' does not exist on struct '{struct_name}'.\n\n\
                                 Why this happens:\n\
                                 You attempted to access or assign to a field that was not defined in the struct's layout.\n\n\
                                 How to resolve:\n\
                                 1. Check the definition of 'make {struct_name}' to see if the field is correctly spelled.\n\
                                 2. Add the field to the struct definition if it is missing:\n\
                                    make {struct_name}\n\
                                        {field} = <default_value>\n\
                                 ================================================================================\n"
                            ));
                        }
                    } else {
                        return Err(format!(
                            "\n================================================================================\n\
                             [KRAiT UNDEFiNED STRUCT ERROR]\n\
                             ================================================================================\n\
                             Struct  : '{struct_name}'\n\
                             Issue   : Struct '{struct_name}' is not defined.\n\n\
                             Why this happens:\n\
                             You are trying to assign to a field on a struct type that has not been declared.\n\n\
                             How to resolve:\n\
                             1. Define the struct prior to usage using:\n\
                                make {struct_name}\n\
                                    field_name = default_value\n\
                             2. Ensure you have imported the module defining '{struct_name}' or check for typos.\n\
                             ================================================================================\n"
                        ));
                    }
                } else {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT TYPE ERROR]\n\
                         ================================================================================\n\
                         Field   : '{field}'\n\
                         Type    : {obj_ty:?}\n\
                         Issue   : Attempted to access/assign field '{field}' on a non-struct type '{obj_ty:?}'.\n\n\
                         Why this happens:\n\
                         Field access (using the dot operator like `object.field`) is only valid on user-defined structs.\n\
                         Primitives (ints, floats, strings, booleans) do not have fields.\n\n\
                         How to resolve:\n\
                         1. Ensure the variable you are accessing is a struct instance.\n\
                         ================================================================================\n"
                    ));
                }
            }
            Stmt::When { cond, then_branch } => {
                let cond_ty = self.infer_type(cond)?;
                if cond_ty != Type::Bool {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT CONDITiON TYPE ERROR]\n\
                         ================================================================================\n\
                         Type    : {cond_ty:?}\n\
                         Issue   : The condition of a 'when' (if) statement must evaluate to a boolean (true/false).\n\n\
                         Why this happens:\n\
                         Control flow branches in Krait expect strict boolean checks to prevent unexpected branch execution and bugs.\n\n\
                         How to resolve:\n\
                         1. Ensure the condition expression evaluates to true or false.\n\
                         2. If comparing values, use equality or comparison operators like `==`, `>`, `<`.\n\
                         ================================================================================\n"
                    ));
                }
                self.enter_scope();
                for s in then_branch { self.analyze_statement(s)?; }
                self.exit_scope();
            }
            Stmt::Repeat { count, body } => {
                let count_ty = self.infer_type(count)?;
                if count_ty != Type::Int {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT LOOP COUNT TYPE ERROR]\n\
                         ================================================================================\n\
                         Type    : {count_ty:?}\n\
                         Issue   : The repeat count of a loop must be an integer.\n\n\
                         Why this happens:\n\
                         Loop iteration counts must be whole numbers. Floating-point numbers, strings, or booleans cannot be used to specify repetition limits.\n\n\
                         How to resolve:\n\
                         1. Ensure the expression after 'repeat' evaluates to an integer (e.g. `5`, or an integer variable).\n\
                         ================================================================================\n"
                    ));
                }
                self.enter_scope();
                for s in body { self.analyze_statement(s)?; }
                self.exit_scope();
            }
            Stmt::Show(expr) | Stmt::Return(Some(expr)) | Stmt::Expr(expr) => {
                self.infer_type(expr)?;
            }
            Stmt::Return(None) | Stmt::StructDef { .. } | Stmt::ExternDecl { .. } | Stmt::Import(_) => {}
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
            Expr::Variable(name) => {
                self.check_moved(name)?;
                self.resolve(name)
                    .ok_or_else(|| format!(
                        "\n================================================================================\n\
                         [KRAiT SCOPE ERROR]\n\
                         ================================================================================\n\
                         Variable: '{name}'\n\
                         Issue   : Variable '{name}' is used before it is declared.\n\n\
                         Why this happens:\n\
                         Krait requires all variables to be explicitly declared before they are referenced.\n\
                         This prevents access to uninitialized memory and avoids common typo-related bugs.\n\n\
                         How to resolve:\n\
                         1. Declare the variable prior to this line using: `set {name} = <value>`\n\
                         2. Check for spelling errors or capitalization mismatches in the variable name.\n\
                         ================================================================================\n"
                    ))
            },
            Expr::FieldAccess { object, field } => {
                let obj_ty = self.infer_type(object)?;
                if let Type::Struct(struct_name) = obj_ty {
                    if let Some(def) = self.struct_definitions.get(&struct_name) {
                        if let Some(field_ty) = def.get(field) {
                            Ok(field_ty.clone())
                        } else {
                            return Err(format!(
                                "\n================================================================================\n\
                                 [KRAiT STRUCT FiELD ERROR]\n\
                                 ================================================================================\n\
                                 Struct  : '{struct_name}'\n\
                                 Field   : '{field}'\n\
                                 Issue   : Field '{field}' does not exist on struct '{struct_name}'.\n\n\
                                 Why this happens:\n\
                                 You attempted to access or assign to a field that was not defined in the struct's layout.\n\n\
                                 How to resolve:\n\
                                 1. Check the definition of 'make {struct_name}' to see if the field is correctly spelled.\n\
                                 2. Add the field to the struct definition if it is missing:\n\
                                    make {struct_name}\n\
                                        {field} = <default_value>\n\
                                 ================================================================================\n"
                            ));
                        }
                    } else {
                        return Err(format!(
                            "\n================================================================================\n\
                             [KRAiT UNDEFiNED STRUCT ERROR]\n\
                             ================================================================================\n\
                             Struct  : '{struct_name}'\n\
                             Issue   : Struct '{struct_name}' is not defined.\n\n\
                             Why this happens:\n\
                             You are trying to reference a field on a struct type that has not been declared.\n\n\
                             How to resolve:\n\
                             1. Define the struct prior to usage using:\n\
                                make {struct_name}\n\
                                    field_name = default_value\n\
                             2. Ensure you have imported the module defining '{struct_name}' or check for typos.\n\
                             ================================================================================\n"
                        ));
                    }
                } else {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT TYPE ERROR]\n\
                         ================================================================================\n\
                         Field   : '{field}'\n\
                         Type    : {obj_ty:?}\n\
                         Issue   : Attempted to access field '{field}' on a non-struct type '{obj_ty:?}'.\n\n\
                         Why this happens:\n\
                         Field access (using the dot operator like `object.field`) is only valid on user-defined structs.\n\
                         Primitives (ints, floats, strings, booleans) do not have fields.\n\n\
                         How to resolve:\n\
                         1. Ensure the variable you are accessing is a struct instance.\n\
                         2. Check if you meant to perform a different operation or if you are using the wrong variable.\n\
                         ================================================================================\n"
                    ));
                }
            }
            Expr::New(name) => {
                if self.struct_definitions.contains_key(name) {
                    Ok(Type::Struct(name.clone()))
                } else {
                    Err(format!(
                        "\n================================================================================\n\
                         [KRAiT UNDEFiNED STRUCT ERROR]\n\
                         ================================================================================\n\
                         Struct  : '{name}'\n\
                         Issue   : Struct '{name}' is not defined.\n\n\
                         Why this happens:\n\
                         You are trying to instantiate a struct type that has not been declared.\n\n\
                         How to resolve:\n\
                         1. Define the struct prior to instantiation using:\n\
                            make {name}\n\
                                field_name = default_value\n\
                         2. Ensure you have imported the module defining '{name}' or check for typos.\n\
                         ================================================================================\n"
                    ))
                }
            }
            Expr::Binary { left, op, right } => {
                let lt = self.infer_type(left)?;
                let rt = self.infer_type(right)?;
                if lt != rt {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT TYPE MISMATCH ERROR]\n\
                         ================================================================================\n\
                         Types   : {lt:?} vs {rt:?}\n\
                         Operator: {op:?}\n\
                         Issue   : Mismatched types in binary operation.\n\n\
                         Why this happens:\n\
                         Krait is a strongly, statically typed language. It does not perform implicit type coercion\n\
                         (like converting float to int automatically) to ensure maximum safety, CPU efficiency,\n\
                         and predictability.\n\n\
                         How to resolve:\n\
                         1. Ensure both sides of the operator evaluate to the same type.\n\
                         2. Explicitly cast or convert types if necessary.\n\
                         ================================================================================\n"
                    ));
                }
                match op {
                    Op::Eq | Op::Gt | Op::Lt => Ok(Type::Bool),
                    _ => Ok(lt),
                }
            }
            Expr::Call { callee, args } => {
                for arg in args { self.infer_type(arg)?; }
                if !self.function_signatures.contains_key(callee) {
                    return Err(format!(
                        "\n================================================================================\n\
                         [KRAiT UNDEFiNED FUNCTiON ERROR]\n\
                         ================================================================================\n\
                         Function: '{callee}'\n\
                         Issue   : Attempted to call a function '{callee}' that is not defined in this scope.\n\n\
                         Why this happens:\n\
                         The compiler could not find any function definition matching the name '{callee}' in the\n\
                         current file or any imported modules.\n\n\
                         How to resolve:\n\
                         1. Define the function using:\n\
                            make {callee}(params)\n\
                                ...\n\
                         2. Verify that you have imported the module containing '{callee}' (e.g., `import io`).\n\
                         3. Check for spelling or casing typos in the function name.\n\
                         ================================================================================\n"
                    ));
                }
                Ok(Type::Int)
            }
        }
    }
}