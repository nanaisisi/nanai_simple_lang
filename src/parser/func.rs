use crate::ast::{Expr, Stmt};
use crate::lexer::Token;

pub fn parse_funcdef(tokens: &[Token], pos: &mut usize) -> Stmt {
    // pub fn/fn <name>(<params>) { <body> }
    if let Some(Token::Pub) = tokens.get(*pos) {
        *pos += 1; // pubは現状無視
    }
    if let Some(Token::Fn) = tokens.get(*pos) {
        *pos += 1;
    } else {
        return Stmt::Error("fnキーワードが必要です".to_string());
    }
    let name = if let Some(Token::Ident(n)) = tokens.get(*pos) {
        *pos += 1;
        n.clone()
    } else {
        return Stmt::Error("関数名が必要です".to_string());
    };
    if tokens.get(*pos) != Some(&Token::LParen) {
        return Stmt::Error("( が必要です".to_string());
    }
    *pos += 1;
    let mut params = Vec::new();
    while let Some(tok) = tokens.get(*pos) {
        match tok {
            Token::Ident(param) => {
                params.push(param.clone());
                *pos += 1;
                if tokens.get(*pos) == Some(&Token::Comma) {
                    *pos += 1;
                }
            }
            Token::RParen => {
                *pos += 1;
                break;
            }
            _ => {
                return Stmt::Error("引数リストが不正です".to_string());
            }
        }
    }
    if tokens.get(*pos) != Some(&Token::LBrace) {
        return Stmt::Error("{ が必要です".to_string());
    }
    *pos += 1;
    // bodyは複数文対応: { stmt1; stmt2; ... }
    let mut stmts = Vec::new();
    while let Some(tok) = tokens.get(*pos) {
        if let Token::RBrace = tok {
            *pos += 1;
            break;
        }
        if let Some(stmt) = crate::parser::print::parse_print(tokens, pos) {
            stmts.push(stmt);
        } else if tokens.get(*pos) == Some(&Token::Let) {
            stmts.push(crate::parser::let_stmt::parse_let(tokens, pos));
        } else {
            let expr = crate::parser::expr::parse_expr(tokens, pos);
            stmts.push(crate::ast::Stmt::Expr(expr));
        }
    }
    let body = Box::new(crate::ast::Expr::Block(stmts));
    Stmt::FuncDef { name, params, body }
}
