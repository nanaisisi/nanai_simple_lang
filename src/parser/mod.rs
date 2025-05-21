mod expr;
mod func;
mod import;
mod let_stmt;
mod print;

use crate::ast::Stmt;
use crate::lexer::Token;
use expr::parse_expr;
use func::parse_funcdef;
use import::parse_import;
use let_stmt::parse_let;
use print::parse_print;

pub fn parse(tokens: &[Token]) -> Vec<Stmt> {
    if tokens.iter().any(|t| matches!(t, Token::Error(_))) {
        let msg = tokens.iter().find_map(|t| {
            if let Token::Error(m) = t {
                Some(m.clone())
            } else {
                None
            }
        });
        return vec![Stmt::Error(
            msg.unwrap_or_else(|| "字句解析エラー".to_string()),
        )];
    }

    if tokens.iter().any(|t| matches!(t, Token::EOF)) && tokens.len() > 1 {
        return vec![Stmt::Error(
            "字句解析エラー: 不正な文字が含まれています".to_string(),
        )];
    }

    let mut pos = 0;
    let mut stmts = Vec::new();
    while tokens.get(pos) != Some(&Token::EOF) {
        if let Some(stmt) = parse_import(tokens, &mut pos) {
            stmts.push(stmt);
        } else if let Some(expr) = parse_print(tokens, &mut pos) {
            stmts.push(expr);
        } else if tokens.get(pos) == Some(&Token::Let) {
            stmts.push(parse_let(tokens, &mut pos));
        } else if tokens.get(pos) == Some(&Token::Pub) {
            stmts.push(parse_funcdef(tokens, &mut pos));
        } else if tokens.get(pos) == Some(&Token::Fn) {
            stmts.push(parse_funcdef(tokens, &mut pos));
        } else {
            let expr = parse_expr(tokens, &mut pos);
            stmts.push(Stmt::Expr(expr));
        }
    }
    stmts
}
