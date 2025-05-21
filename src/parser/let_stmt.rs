use crate::ast::{Expr, Stmt};
use crate::lexer::Token;

pub fn parse_let(tokens: &[Token], pos: &mut usize) -> Stmt {
    // let/mut <name> [: <type>] = <expr>;
    *pos += 1; // let
    let mut mutable = false;
    if tokens.get(*pos) == Some(&Token::Mut) {
        mutable = true;
        *pos += 1;
    }
    let name = if let Some(Token::Ident(n)) = tokens.get(*pos) {
        *pos += 1;
        n.clone()
    } else {
        return Stmt::Error("変数名が必要です".to_string());
    };
    let mut ty = None;
    if tokens.get(*pos) == Some(&Token::Colon) {
        *pos += 1;
        if let Some(Token::Ident(t)) = tokens.get(*pos) {
            ty = Some(t.clone());
            *pos += 1;
        } else {
            return Stmt::Error(": の後に型名が必要です".to_string());
        }
    }
    if tokens.get(*pos) != Some(&Token::Eq) {
        return Stmt::Error("= が必要です".to_string());
    }
    *pos += 1;
    // 値は式としてパース（仮）
    let value = Expr::Number(0); // TODO: parse_exprで本来はパース
    // ; をスキップ
    if tokens.get(*pos) == Some(&Token::EOF) {
        // 終端
    } else {
        *pos += 1;
    }
    Stmt::Let {
        name,
        value,
        mutable,
        ty,
    }
}
