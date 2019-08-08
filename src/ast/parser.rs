use super::tigerabs::{Exp, _Exp};
use super::position::{Pos, WithPos};
use lalrpop_util::lalrpop_mod;

#[macro_use]
lalrpop_mod!(pub parser);

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Pos),
}

pub fn parse(source : String) -> Result<Exp, ParseError> {
    let str_src: &str = &*source;
    let box_exp = parser::ExprParser::new().parse(str_src).unwrap(); // esto hace un panic, que no es lo que queremos

     Ok(*box_exp)
}
