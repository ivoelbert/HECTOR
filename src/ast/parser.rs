use super::{AST};
use super::position::{Pos};
use super::lexer::Lexer;
use lalrpop_util::lalrpop_mod;
use serde::{Serialize};

#[macro_use]
lalrpop_mod!(pub parser);

#[derive(Debug, Serialize)]
pub enum ParseError {
    UnexpectedToken(Pos),
}

pub fn parse(str_src : &str) -> Result<AST, ParseError> {
    let lexed = Lexer::new(str_src.lines());

    match parser::ExprParser::new().parse(lexed) {
        Ok(box_exp) => Ok(*box_exp),
        Err(..) => {
            Err(ParseError::UnexpectedToken(Pos {column: 0, line: 0}))
        }
    }
}