use super::tigerabs::{Exp, _Exp};
use super::position::{Pos, WithPos};
use super::lexer::Lexer;
use std::fs::{read_dir, read_to_string};
//use lalrpop_util::lalrpop_mod;

//#[macro_use]
//lalrpop_mod!(pub parser);

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Pos),
}


pub fn parse(source : String) -> Result<Exp, ParseError> {
    Err(ParseError::UnexpectedToken(Pos::new(0, 0)))
    //let str_src: &str = &*source;
    //let box_exp = parser::ExprParser::new().parse(str_src).unwrap();

    //return Ok(*box_exp)
}

#[test]
fn basic_full_tokens() {
    let good_path = "./tiger_sources/lexer/simplest.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        println!("{:?}", tok);
    }
}

#[test]
fn basic_compound_tokens() {
    let good_path = "./tiger_sources/lexer/compound_tokens.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        println!("{:?}", tok);
    }
}
