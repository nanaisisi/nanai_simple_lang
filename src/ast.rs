// ASTノード定義
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
    Call(String, Vec<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
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
