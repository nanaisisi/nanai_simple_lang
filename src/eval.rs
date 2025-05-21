use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

pub fn eval_stmts(stmts: &[Stmt]) -> i64 {
    let mut funcs: HashMap<String, (Vec<String>, Expr)> = HashMap::new();
    let mut vars: HashMap<String, i64> = HashMap::new();
    let mut last_result = 0;

    // 標準関数テーブル
    let mut std_funcs: HashMap<String, fn(Vec<i64>) -> i64> = HashMap::new();
    std_funcs.insert("print".to_string(), |args| {
        println!("{}", args[0]);
        args[0]
    });

    for stmt in stmts {
        match stmt {
            Stmt::Import(filename) => {
                use std::fs;
                let code = fs::read_to_string(filename).expect("importファイルが読み込めません");
                let tokens = crate::lexer::tokenize(&code);
                let imported_stmts = crate::parser::parse(&tokens);
                // 再帰的にimportを評価
                eval_stmts(&imported_stmts);
            }
            Stmt::FuncDef { name, params, body } => {
                funcs.insert(name.clone(), (params.clone(), *body.clone()));
            }
            Stmt::Let { name, value, .. } => {
                let v = eval_expr(value, &funcs, &vars, &std_funcs);
                vars.insert(name.clone(), v);
                last_result = v;
            }
            Stmt::Expr(expr) => {
                last_result = eval_expr(expr, &funcs, &vars, &std_funcs);
            }
            Stmt::Print(expr) => {
                let v = eval_expr(expr, &funcs, &vars, &std_funcs);
                std_funcs["print"](vec![v]);
                last_result = v;
            }
            Stmt::Error(msg) => {
                eprintln!("[解析エラー] {}", msg);
                last_result = 0;
            }
        }
    }
    last_result
}

fn eval_expr(
    expr: &Expr,
    funcs: &HashMap<String, (Vec<String>, Expr)>,
    vars: &HashMap<String, i64>,
    std_funcs: &HashMap<String, fn(Vec<i64>) -> i64>,
) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => {
            eval_expr(lhs, funcs, vars, std_funcs) + eval_expr(rhs, funcs, vars, std_funcs)
        }
        Expr::Var(name) => *vars.get(name).expect("未定義の変数"),
        Expr::Call(name, args) => {
            if let Some(f) = std_funcs.get(name) {
                let arg_vals = args
                    .iter()
                    .map(|a| eval_expr(a, funcs, vars, std_funcs))
                    .collect();
                f(arg_vals)
            } else {
                let (params, body) = funcs.get(name).expect("未定義の関数");
                if params.len() != args.len() {
                    panic!("引数の数が一致しません");
                }
                let mut new_vars = HashMap::new();
                for (p, a) in params.iter().zip(args.iter()) {
                    new_vars.insert(p.clone(), eval_expr(a, funcs, vars, std_funcs));
                }
                eval_expr(body, funcs, &new_vars, std_funcs)
            }
        }
    }
}
