use super::{AST, posed_exp, Exp};
use super::position::{Pos};
use lalrpop_util::lalrpop_mod;

//#[macro_use]
//lalrpop_mod!(pub parser);

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Pos),
}

pub fn parse(source : String) -> Result<AST, ParseError> {
    let str_src: &str = &*source;
    let box_exp = posed_exp(Exp::Unit, 1, 1);

     Ok(*box_exp)
}
