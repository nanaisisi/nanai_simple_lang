// 字句解析（トークナイザー）
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    StringLiteral(String),
    Error(String),
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    // BOM（Byte Order Mark）があればスキップ
    if let Some('\u{feff}') = input.chars().next() {
        let mut chars = input.chars();
        chars.next(); // skip BOM
        return tokenize(&chars.collect::<String>());
    }

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
            '"' => {
                chars.next(); // skip opening quote
                let mut s = String::new();
                while let Some(&d) = chars.peek() {
                    if d == '"' {
                        chars.next();
                        break;
                    } else {
                        s.push(d);
                        chars.next();
                    }
                }
                tokens.push(Token::StringLiteral(s));
            }
            '/' => {
                chars.next();
                if let Some(&'/') = chars.peek() {
                    // 行コメント: // ～行末までスキップ
                    while let Some(&d) = chars.peek() {
                        chars.next();
                        if d == '\n' {
                            break;
                        }
                    }
                } else {
                    // 単独の/は不正文字扱い
                    let pos = input.len() - chars.clone().count();
                    let context: String =
                        input.chars().skip(pos.saturating_sub(5)).take(10).collect();
                    tokens.push(Token::Error(format!(
                        "不正な文字: '/' (U+002F) 位置: {} 付近: '{}'",
                        pos, context
                    )));
                }
            }
            ' ' | '\n' | '\r' | '\t' => {
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
                let pos = input.len() - chars.clone().count();
                let context: String = input.chars().skip(pos.saturating_sub(5)).take(10).collect();
                tokens.push(Token::Error(format!(
                    "不正な文字: '{}' (U+{:04X}) 位置: {} 付近: '{}'",
                    c, c as u32, pos, context
                )));
                chars.next();
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}
