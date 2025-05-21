use std::env;
use std::fs;
use nanai_simple_lang::lexer::tokenize;
use nanai_simple_lang::parser::parse;
use nanai_simple_lang::eval::eval_stmts;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: nasl <file.nasl>");
        std::process::exit(1);
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("ファイルが読み込めません");
    let tokens = tokenize(&code);
    let stmts = parse(&tokens);
    let result = eval_stmts(&stmts);
    println!("結果: {}", result);
}
