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
                }
            }
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            // UTF-8 BOMや制御文字、ASCII範囲外の文字はスキップ（エラーにしない）
            c if (c as u32) > 0x7F || c.is_control() => {
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
                    // "import"は予約語から除外
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            // どんな文字でも未対応記号はすべてスキップ（エラーにしない）
            _ => {
                chars.next();
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}
