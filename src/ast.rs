#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum Op {
    Plus, Minus, Mul, Div, Eq, Gt, Lt
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    VariableDecl {
        name: String,
        value: Expr,
    },
    When {
        cond: Expr,
        then_branch: Vec<Stmt>,
    },
    Repeat {
        count: Expr,
        body: Vec<Stmt>,
    },
    Show(Expr),
    Return(Option<Expr>),
    Expr(Expr),
}