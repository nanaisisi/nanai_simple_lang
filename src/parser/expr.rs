use crate::ast::Expr;
use crate::lexer::Token;

pub fn parse_expr(tokens: &[Token], pos: &mut usize) -> Expr {
    // 加算式: term (+ term)*
    let mut left = parse_term(tokens, pos);
    while let Some(Token::Plus) = tokens.get(*pos) {
        *pos += 1;
        let right = parse_term(tokens, pos);
        left = Expr::Add(Box::new(left), Box::new(right));
    }
    left
}

pub fn parse_term(tokens: &[Token], pos: &mut usize) -> Expr {
    // 数値リテラル or 識別子 or 関数呼び出し
    match tokens.get(*pos) {
        Some(Token::Number(n)) => {
            *pos += 1;
            Expr::Number(*n)
        }
        Some(Token::Ident(name)) => {
            let name = name.clone();
            *pos += 1;
            if tokens.get(*pos) == Some(&Token::LParen) {
                *pos += 1;
                let mut args = Vec::new();
                while tokens.get(*pos) != Some(&Token::RParen) && tokens.get(*pos) != Some(&Token::EOF) {
                    args.push(parse_expr(tokens, pos));
                    if tokens.get(*pos) == Some(&Token::Comma) {
                        *pos += 1;
                    } else {
                        break;
                    }
                }
                if tokens.get(*pos) == Some(&Token::RParen) {
                    *pos += 1;
                }
                Expr::Call(name, args)
            } else {
                Expr::Var(name)
            }
        }
        _ => {
            Expr::Number(0)
        }
    }
}
