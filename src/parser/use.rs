use crate::ast::Stmt;
use crate::lexer::Token;

// use "lib.nasl"; のような構文をサポート
pub fn parse_use(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "use" {
            *pos += 1;
            if let Some(Token::StringLiteral(filename)) = tokens.get(*pos) {
                *pos += 1;
                return Some(Stmt::Import(filename.clone()));
            }
        }
    }
    None
}
