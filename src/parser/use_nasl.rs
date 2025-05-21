use crate::ast::Stmt;
use crate::lexer::Token;

// useはRustの予約語のため、use_nasl.rsというファイル名に。
// Rust風: use lib; use foo::bar; use foo::*; など
// 今後mod.rsや名前空間も拡張可能な設計
pub fn parse_use(tokens: &[Token], pos: &mut usize) -> Option<Stmt> {
    if let Some(Token::Ident(s)) = tokens.get(*pos) {
        if s == "use" {
            *pos += 1;
            // Rust風: use lib; use foo::bar; use foo::*; など
            if let Some(Token::Ident(modname)) = tokens.get(*pos) {
                *pos += 1;
                // use lib; → lib.nasl
                let fname = format!("{}.nasl", modname);
                return Some(Stmt::Import(fname));
            } else if let Some(Token::StringLiteral(filename)) = tokens.get(*pos) {
                *pos += 1;
                // use "lib.nasl"; も許容
                return Some(Stmt::Import(filename.clone()));
            }
        }
    }
    None
}
