use super::tigerabs::{Exp, posed_exp, _Exp};
use super::position::{Pos};
use lalrpop_util::lalrpop_mod;

//#[macro_use]
//lalrpop_mod!(pub parser);

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Pos),
}

pub fn parse(source : String) -> Result<Exp, ParseError> {
    let str_src: &str = &*source;
    let box_exp = posed_exp(_Exp::UnitExp, 1, 1);

     Ok(*box_exp)
}
