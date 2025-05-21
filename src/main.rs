use std::collections::HashMap;
use std::io::{self, Write};

// トークンの種類
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i64),
    Plus,
    Pub,
    Fn,
    Ident(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Let,
    Mut,
    Colon,
    Eq,
    EOF,
}

// 字句解析
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = 0;
                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) {
                        num = num * 10 + d.to_digit(10).unwrap() as i64;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RBrace);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Eq);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        ident.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "mut" => tokens.push(Token::Mut),
                    "pub" => tokens.push(Token::Pub),
                    "fn" => tokens.push(Token::Fn),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            _ => {
                panic!("不正な文字: {}", c);
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}

// ASTノード
#[derive(Debug, Clone)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
enum Stmt {
    Expr(Expr),
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    Let {
        name: String,
        value: Expr,
        mutable: bool,
        ty: Option<String>,
    },
}

// 構文解析
fn parse(tokens: &[Token]) -> Vec<Stmt> {
    let mut pos = 0;
    let mut stmts = Vec::new();
    while tokens.get(pos) != Some(&Token::EOF) {
        if tokens.get(pos) == Some(&Token::Let) {
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
    // pubがあればスキップ
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

fn eval_stmts(stmts: &[Stmt]) -> i64 {
    let mut funcs: HashMap<String, (Vec<String>, Expr)> = HashMap::new();
    let mut vars: HashMap<String, i64> = HashMap::new();
    let mut last_result = 0;
    for stmt in stmts {
        match stmt {
            Stmt::FuncDef { name, params, body } => {
                funcs.insert(name.clone(), (params.clone(), *body.clone()));
            }
            Stmt::Let { name, value, .. } => {
                let v = eval_expr(value, &funcs, &vars);
                vars.insert(name.clone(), v);
                last_result = v;
            }
            Stmt::Expr(expr) => {
                last_result = eval_expr(expr, &funcs, &vars);
            }
        }
    }
    last_result
}

fn eval_expr(
    expr: &Expr,
    funcs: &HashMap<String, (Vec<String>, Expr)>,
    vars: &HashMap<String, i64>,
) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => eval_expr(lhs, funcs, vars) + eval_expr(rhs, funcs, vars),
        Expr::Var(name) => *vars.get(name).expect("未定義の変数"),
        Expr::Call(name, args) => {
            let (params, body) = funcs.get(name).expect("未定義の関数");
            if params.len() != args.len() {
                panic!("引数の数が一致しません");
            }
            let mut new_vars = HashMap::new();
            for (p, a) in params.iter().zip(args.iter()) {
                new_vars.insert(p.clone(), eval_expr(a, funcs, vars));
            }
            eval_expr(body, funcs, &new_vars)
        }
    }
}

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
