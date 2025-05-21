use nanai_simple_lang::eval::eval_stmts;
use nanai_simple_lang::lexer::tokenize;
use nanai_simple_lang::parser::parse;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: nasl <file.nasl>");
        std::process::exit(1);
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("ファイルが読み込めません");
    let tokens = tokenize(&code);
    let mut stmts = parse(&tokens);
    // main関数が定義されていれば自動で main() を呼び出す
    let has_main = stmts
        .iter()
        .any(|s| matches!(s, nanai_simple_lang::ast::Stmt::FuncDef { name, .. } if name == "main"));
    if has_main {
        use nanai_simple_lang::ast::{Expr, Stmt};
        stmts.push(Stmt::Expr(Expr::Call("main".to_string(), vec![])));
    }
    let result = eval_stmts(&stmts);
    println!("結果: {}", result);
}
