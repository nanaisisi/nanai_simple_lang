use crate::ast::Stmt;
use crate::lexer::Token;

pub fn parse_import(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    // import "filename"
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "import" {
            *pos += 1;
            if let Some(Token::StringLiteral(filename)) = tokens.get(*pos) {
                *pos += 1;
                return Some(Stmt::Import(filename.clone()));
            }
        }
    }
    None
}
