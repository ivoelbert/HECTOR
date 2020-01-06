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

<<<<<<< HEAD
pub fn parse(source : String) -> Result<AST, ParseError> {
    let str_src: &str = &*source;
    let lexed = Lexer::new(str_src.lines());
=======
pub fn parse(source : &str) -> Result<AST, ParseError> {
    let lexed = Lexer::new(source.lines());
    let box_exp = parser::ExprParser::new().parse(lexed).unwrap();
>>>>>>> serialization

    match parser::ExprParser::new().parse(lexed) {
        Ok(box_exp) => Ok(*box_exp),
        Err(e) => {
            println!("{:?}", e);
            Err(ParseError::UnexpectedToken(Pos {column: 0, line: 0}))
        }
    }
}

#[test]
fn parsed_test() {
    let good_path = "./tiger_sources/good/queens.tig";
    let contents: String = read_to_string(&good_path).unwrap();

    match parse(contents) {
        Ok(_) => println!("\n\nVamo ñubel\n\n"),
        _ => println!("\n\n:(\n\n"),
    }

    return;
}

#[test]
fn lexed_test() {
    let good_path = "./tiger_sources/good/queens.tig";
    let contents: String = read_to_string(&good_path).unwrap();

    let str_src: &str = &*contents;
    let lexed = Lexer::new(str_src.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    return;
}
