use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

pub type StdFunc = fn(Vec<i64>) -> i64;

fn print_fn(args: Vec<i64>) -> i64 {
    for v in args {
        println!("{}", v);
    }
    0
}

pub fn get_std_funcs() -> HashMap<String, StdFunc> {
    let mut map = HashMap::new();
    map.insert("print".to_string(), print_fn as fn(Vec<i64>) -> i64);
    map.insert("input".to_string(), |_args| {
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        // 入力値をi64に変換して返す（失敗時は0）
        buf.trim().parse::<i64>().unwrap_or(0)
    });
    map
}

pub fn eval_stmts(stmts: &[Stmt]) -> i64 {
    let mut funcs: HashMap<String, (Vec<String>, Expr)> = HashMap::new();
    let vars: HashMap<String, i64> = HashMap::new();
    let mut last_result = 0;

    // 標準関数テーブル
    let std_funcs = get_std_funcs();

    for stmt in stmts {
        match stmt {
            Stmt::Import(filename) => {
                use std::fs;
                let code = fs::read_to_string(filename).expect("importファイルが読み込めません");
                let tokens = crate::lexer::tokenize(&code);
                let imported_stmts = crate::parser::parse(&tokens);
                // 関数定義をマージ
                for s in &imported_stmts {
                    if let Stmt::FuncDef { name, params, body } = s {
                        funcs.insert(name.clone(), (params.clone(), *body.clone()));
                    }
                }
                // 再帰的にimportを評価（副作用目的）
                eval_stmts(&imported_stmts);
            }
            Stmt::FuncDef { name, params, body } => {
                funcs.insert(name.clone(), (params.clone(), *body.clone()));
            }
            Stmt::Let { name: _, value, .. } => {
                let v = eval_expr(value, &funcs, &vars, &std_funcs);
                // name未使用のため警告回避
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
            Stmt::StructDef { name: _, fields: _ } => {
                // 構造体定義は未実装
            }
        }
    }
    last_result
}

fn eval_expr(
    expr: &Expr,
    funcs: &HashMap<String, (Vec<String>, Expr)>,
    vars: &HashMap<String, i64>,
    std_funcs: &HashMap<String, StdFunc>,
) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Bool(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        Expr::Str(_) => 0, // 文字列型は未対応
        Expr::Add(lhs, rhs) => {
            eval_expr(lhs, funcs, vars, std_funcs) + eval_expr(rhs, funcs, vars, std_funcs)
        }
        Expr::Var(name) => *vars.get(name).unwrap_or(&0),
        Expr::Call(name, args) => {
            if let Some(f) = std_funcs.get(name) {
                let arg_vals = args
                    .iter()
                    .map(|a| eval_expr(a, funcs, vars, std_funcs))
                    .collect();
                f(arg_vals)
            } else if let Some((params, body)) = funcs.get(name) {
                if params.len() != args.len() {
                    panic!("引数の数が一致しません");
                }
                let mut new_vars = std::collections::HashMap::new();
                for (p, a) in params.iter().zip(args.iter()) {
                    new_vars.insert(p.clone(), eval_expr(a, funcs, vars, std_funcs));
                }
                eval_expr(body, funcs, &new_vars, std_funcs)
            } else {
                panic!("未定義の関数: {}", name);
            }
        }
        Expr::Block(stmts) => {
            let mut last = 0;
            for stmt in stmts {
                match stmt {
                    Stmt::Expr(e) => last = eval_expr(e, funcs, vars, std_funcs),
                    Stmt::Print(e) => {
                        last = eval_expr(e, funcs, vars, std_funcs);
                        std_funcs["print"](vec![last]);
                    }
                    Stmt::Let { value, .. } => {
                        last = eval_expr(value, funcs, vars, std_funcs);
                    }
                    _ => {}
                }
            }
            last
        }
        Expr::If {
            cond,
            then_branch,
            else_branch,
        } => {
            if eval_expr(cond, funcs, vars, std_funcs) != 0 {
                eval_expr(then_branch, funcs, vars, std_funcs)
            } else if let Some(else_b) = else_branch {
                eval_expr(else_b, funcs, vars, std_funcs)
            } else {
                0
            }
        }
        Expr::For {
            var,
            start,
            end,
            body,
        } => {
            let s = eval_expr(start, funcs, vars, std_funcs);
            let e = eval_expr(end, funcs, vars, std_funcs);
            let mut last = 0;
            for i in s..e {
                let mut new_vars = vars.clone();
                new_vars.insert(var.clone(), i);
                last = eval_expr(body, funcs, &new_vars, std_funcs);
            }
            last
        }
        _ => 0,
    }
}
