use crate::ast::{Expr, Stmt};
use crate::lexer::Token;

// import "filename.nasl" に対応
fn parse_import(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "import" {
            *pos += 1;
            if let Some(Token::StringLiteral(filename)) = tokens.get(*pos) {
                *pos += 1;
                return Some(Stmt::Import(filename.clone()));
            } else {
                panic!("importの後にファイル名（文字列リテラル）が必要");
            }
        }
    }
    None
}

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

fn parse_funcdef(tokens: &[Token], pos: &mut usize) -> Stmt {
    if tokens.get(*pos) == Some(&Token::Pub) {
        *pos += 1;
    }
    if tokens.get(*pos) != Some(&Token::Fn) {
        panic!("fnが必要");
    }
    *pos += 1; // fn
    let name = if let Token::Ident(ref s) = tokens[*pos] {
        s.clone()
    } else {
        panic!("関数名が必要");
    };
    *pos += 1;
    if tokens.get(*pos) != Some(&Token::LParen) {
        panic!("(");
    }
    *pos += 1;
    let mut params = Vec::new();
    while let Some(Token::Ident(param)) = tokens.get(*pos) {
        params.push(param.clone());
        *pos += 1;
        if tokens.get(*pos) == Some(&Token::Comma) {
            *pos += 1;
        } else {
            break;
        }
    }
    if tokens.get(*pos) != Some(&Token::RParen) {
        panic!("関数引数リストの閉じ括弧が必要");
    }
    *pos += 1;
    if tokens.get(*pos) != Some(&Token::LBrace) {
        panic!("関数本体の{{が必要");
    }
    *pos += 1;
    let body = parse_expr(tokens, pos);
    if tokens.get(*pos) != Some(&Token::RBrace) {
        panic!("関数本体の}}が必要");
    }
    *pos += 1;
    Stmt::FuncDef {
        name,
        params,
        body: Box::new(body),
    }
}

fn parse_let(tokens: &[Token], pos: &mut usize) -> Stmt {
    *pos += 1; // let
    let mut mutable = false;
    if tokens.get(*pos) == Some(&Token::Mut) {
        mutable = true;
        *pos += 1;
    }
    let name = if let Token::Ident(ref s) = tokens[*pos] {
        s.clone()
    } else {
        panic!("変数名が必要");
    };
    *pos += 1;
    let mut ty = None;
    if tokens.get(*pos) == Some(&Token::Colon) {
        *pos += 1;
        if let Token::Ident(ref t) = tokens[*pos] {
            ty = Some(t.clone());
            *pos += 1;
        } else {
            panic!("型名が必要");
        }
    }
    if tokens.get(*pos) != Some(&Token::Eq) {
        panic!("= が必要");
    }
    *pos += 1;
    let value = parse_expr(tokens, pos);
    Stmt::Let {
        name,
        value,
        mutable,
        ty,
    }
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut left = parse_term(tokens, pos);
    while tokens.get(*pos) == Some(&Token::Plus) {
        *pos += 1;
        let right = parse_term(tokens, pos);
        left = Expr::Add(Box::new(left), Box::new(right));
    }
    left
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Expr {
    match &tokens[*pos] {
        Token::Number(n) => {
            *pos += 1;
            Expr::Number(*n)
        }
        Token::Ident(name) => {
            let name = name.clone();
            *pos += 1;
            if tokens.get(*pos) == Some(&Token::LParen) {
                *pos += 1;
                let mut args = Vec::new();
                while tokens.get(*pos) != Some(&Token::RParen) {
                    args.push(parse_expr(tokens, pos));
                    if tokens.get(*pos) == Some(&Token::Comma) {
                        *pos += 1;
                    } else {
                        break;
                    }
                }
                if tokens.get(*pos) != Some(&Token::RParen) {
                    panic!("関数呼び出しの)が必要");
                }
                *pos += 1;
                Expr::Call(name, args)
            } else {
                Expr::Var(name)
            }
        }
        _ => panic!("予期しないトークン: {:?}", tokens[*pos]),
    }
}

fn parse_print(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "print" && tokens.get(*pos + 1) == Some(&Token::LParen) {
            *pos += 2; // print(
            let expr = parse_expr(tokens, pos);
            if tokens.get(*pos) != Some(&Token::RParen) {
                panic!("printの)が必要");
            }
            *pos += 1;
            return Some(Stmt::Print(Box::new(expr)));
        }
    }
    None
}
