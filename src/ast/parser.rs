use super::tigerabs::Exp;
use lalrpop_util::lalrpop_mod;

#[macro_use]
lalrpop_mod!(pub parser);

pub enum ParseError {}

pub fn parse(src: String) -> Result<Exp, ParseError> {
    let str_src: &str = &*src;
    let box_exp = parser::ExprParser::new().parse(str_src).unwrap();

    return Ok(*box_exp)
}