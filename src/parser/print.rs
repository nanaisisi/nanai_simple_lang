use crate::ast::Expr;
use crate::ast::Stmt;
use crate::lexer::Token;

pub fn parse_print(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    // print ( <expr> )
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "print" {
            *pos += 1;
            if tokens.get(*pos) == Some(&Token::LParen) {
                *pos += 1;
                // 仮: 数値リテラルのみ対応
                let expr = if let Some(Token::Number(n)) = tokens.get(*pos) {
                    *pos += 1;
                    Expr::Number(*n)
                } else {
                    Expr::Number(0)
                };
                if tokens.get(*pos) == Some(&Token::RParen) {
                    *pos += 1;
                    return Some(Stmt::Print(Box::new(expr)));
                }
            }
        }
    }
    None
}
