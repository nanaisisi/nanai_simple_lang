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
    // 値は式としてパース
    let value = crate::parser::expr::parse_expr(tokens, pos);
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

#[cfg(feature = "nom")]
pub mod nom_let_parser {
    use super::*;
    use nom::{
        IResult,
        bytes::complete::tag,
        character::complete::{alpha1, alphanumeric1, multispace0},
        combinator::{opt, recognize},
        multi::many1,
        sequence::tuple,
    };

    // 入力: &str, 出力: (mutable, name, ty, value)
    pub fn parse_let(input: &str) -> IResult<&str, (bool, String, Option<String>, String)> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("let")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, mut_kw) = opt(tag("mut"))(input)?;
        let mutable = mut_kw.is_some();
        let (input, _) = multispace0(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = multispace0(input)?;
        let (input, ty) = opt(tuple((tag(":"), multispace0, alpha1)))(input)?;
        let ty = ty.map(|(_, _, t)| t.to_string());
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = multispace0(input)?;
        // 値はここでは文字列としてパース（本来は式パーサを呼ぶ）
        let (input, value) = recognize(many1(alphanumeric1))(input)?;
        Ok((input, (mutable, name.to_string(), ty, value.to_string())))
    }
}
