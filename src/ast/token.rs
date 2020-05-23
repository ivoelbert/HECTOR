//! Source code will be divided in tokens while parsing
//! Tokens can be single characters, reserved word, identifiers or literals

use std::fmt::{self, Display, Formatter};

use super::position::Pos;

#[allow(dead_code)]
#[derive(Debug)]
/// The token variants
pub enum Tok {
    /// &
    Ampersand,
    /// reserved word
    Array,
    /// reserved word
    Break,
    /// }
    CloseCurly,
    /// )
    CloseParen,
    /// ]
    CloseSquare,
    /// :
    Colon,
    /// :=
    ColonEqual,
    /// ,
    Comma,
    /// reserved word
    Do,
    /// .
    Dot,
    /// reserved word
    Else,
    /// reserved word
    End,
    /// =
    Equal,
    /// reserved word
    For,
    /// reserved word
    Function,
    /// >
    Greater,
    /// >=
    GreaterOrEqual,
    /// An identifier
    Ident(String),
    /// reserved word
    If,
    /// reserved word
    In,
    /// A number literal
    Int(i32),
    /// <
    Lesser,
    /// <=
    LesserOrEqual,
    /// reserved word
    Let,
    /// -
    Minus,
    /// reserved word
    Nil,
    /// !=
    NotEqual,
    /// reserved word
    Of,
    /// {
    OpenCurly,
    /// (
    OpenParen,
    /// [
    OpenSquare,
    /// |
    Pipe,
    /// +
    Plus,
    /// ;
    Semicolon,
    /// /
    Slash,
    /// *
    Star,
    /// A string literal
    Str(String),
    /// reserved word
    Then,
    /// reserved word
    To,
    /// reserved word
    Type,
    /// reserved word
    Var,
    /// reserved word
    While,
}

#[derive(Debug)]
/// A code appearing in the source code
pub struct Token {
    /// Position in code
    pub start: Pos,
    /// Variant
    pub token: Tok,
}

impl Display for Tok {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "token")
    }
}