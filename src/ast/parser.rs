use super::{AST};
use super::position::{Pos, WithPos};
use super::lexer::Lexer;
use std::fs::{read_dir, read_to_string};
use lalrpop_util::lalrpop_mod;

#[macro_use]
lalrpop_mod!(pub parser);

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Pos),
}

pub fn parse(str_src : &str) -> Result<AST, ParseError> {
    let lexed = Lexer::new(str_src.lines());

    match parser::ExprParser::new().parse(lexed) {
        Ok(box_exp) => Ok(*box_exp),
        Err(e) => {
            Err(ParseError::UnexpectedToken(Pos {column: 0, line: 0}))
        }
    }
}