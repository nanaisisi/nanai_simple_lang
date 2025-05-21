mod expr;
mod func;
mod let_stmt;
mod print;
// useはRustの予約語のため、use_nasl.rsというファイル名に。
mod use_nasl;

use crate::ast::Stmt;
use crate::lexer::Token;
use expr::parse_expr;
use func::parse_funcdef;
use let_stmt::parse_let;
use print::parse_print;
// useはRustの予約語のため、use_nasl.rsというファイル名に。
use use_nasl::parse_use;

pub fn parse(tokens: &[Token]) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();
    while tokens.get(pos) != Some(&Token::EOF) {
        if let Some(stmt) = parse_use(tokens, &mut pos) {
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
