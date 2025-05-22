// ASTノード定義
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Str(String),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
    Call(String, Vec<Expr>),
    Block(Vec<Stmt>),
    StructInit(String, Vec<(String, Expr)>),
    FieldAccess(Box<Expr>, String),
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    For {
        var: String,
        start: Box<Expr>,
        end: Box<Expr>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    StructDef {
        name: String,
        fields: Vec<String>,
    },
    Let {
        name: String,
        value: Expr,
        mutable: bool,
        ty: Option<String>,
    },
    Print(Box<Expr>),
    Import(String),
    Error(String),
}
