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

pub fn parse(source : String) -> Result<AST, ParseError> {
    let str_src: &str = &*source;
    let lexed = Lexer::new(str_src.lines());
    let box_exp = parser::ExprParser::new().parse(lexed).unwrap();

    println!("{:?}", box_exp);

    return Ok(*box_exp);
}