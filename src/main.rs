mod ast;
mod eval;
mod lexer;
mod parser;

use eval::eval_stmts;
use lexer::tokenize;
use parser::parse;
use std::io::{self, Write};

fn main() {
    print!("コードを入力してください（例: pub fn add(a, b) {{ a + b }} add(1, 2) ）: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let tokens = tokenize(&input);
    let stmts = parse(&tokens);
    let result = eval_stmts(&stmts);
    println!("結果: {}", result);
}
