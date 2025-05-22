use crate::ast::Expr;
use crate::lexer::Token;

pub fn parse_expr(tokens: &[Token], pos: &mut usize) -> Expr {
    // if式
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "if" {
            *pos += 1;
            let cond = parse_expr(tokens, pos);
            let then_branch = parse_expr(tokens, pos);
            let else_branch = if let Some(Token::Ident(e)) = tokens.get(*pos) {
                if e == "else" {
                    *pos += 1;
                    Some(Box::new(parse_expr(tokens, pos)))
                } else {
                    None
                }
            } else {
                None
            };
            return Expr::If {
                cond: Box::new(cond),
                then_branch: Box::new(then_branch),
                else_branch,
            };
        }
        if s == "for" {
            *pos += 1;
            // for <var> in <start>..<end> <body>
            if let Some(Token::Ident(var)) = tokens.get(*pos) {
                let var = var.clone();
                *pos += 1;
                if tokens.get(*pos) == Some(&Token::Ident("in".to_string())) {
                    *pos += 1;
                    let start = parse_expr(tokens, pos);
                    if tokens.get(*pos) == Some(&Token::Colon) {
                        *pos += 1;
                    }
                    let end = parse_expr(tokens, pos);
                    let body = parse_expr(tokens, pos);
                    return Expr::For {
                        var,
                        start: Box::new(start),
                        end: Box::new(end),
                        body: Box::new(body),
                    };
                }
            }
        }
    }
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
                while tokens.get(*pos) != Some(&Token::RParen)
                    && tokens.get(*pos) != Some(&Token::EOF)
                {
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
        _ => Expr::Number(0),
    }
}
