use std::fmt::{self, Display, Formatter};

use super::position::Pos;

#[derive(Debug)]
pub enum Tok {
    Ampersand,
    Array,
    Break,
    CloseCurly,
    CloseParen,
    CloseSquare,
    Colon,
    ColonEqual,
    Comma,
    Do,
    Dot,
    Else,
    End,
    Equal,
    For,
    Function,
    Greater,
    GreaterOrEqual,
    Ident(String),
    If,
    In,
    Int(i32),
    Lesser,
    LesserOrEqual,
    Let,
    Minus,
    Nil,
    NotEqual,
    Of,
    OpenCurly,
    OpenParen,
    OpenSquare,
    Pipe,
    Plus,
    Semicolon,
    Slash,
    Star,
    Str(String),
    Then,
    To,
    Type,
    Var,
    While,
}

#[derive(Debug)]
pub struct Token {
    pub start: Pos,
    pub token: Tok,
}

impl Display for Tok {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "token")
    }
}