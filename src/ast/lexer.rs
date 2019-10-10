
use regex::Regex;
use std::str::{Lines, SplitWhitespace};

// An OK result means (Column, Token, Line). It's a weird hack to work with LALRPOP's way of doing things
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub enum Tok {
    Point, // .
    Colon, // :
    Assign, // :=
    Comma, // ,
    Semicolon, // ;
    OpenParen, // (
    CloseParen, // )
    OpenBracket, // [
    CloseBracket, // ]
    OpenBraces, // {
    CloseBraces, // }
    Ampersand, // &
    Pipe, // |
    Equals, // =
    Lt, // <
    Lte, // <=
    Gt, // >
    Gte, // >=
    Neq, // <>
    Plus, // +
    Minus, // -
    Times, // *
    Div, // /
    Type, // type
    Array, // array
    Of, // of
    Var, // var
    Function, // function
    Let, // let
    In, // in
    End, // end
    If, // if
    Then, // then
    Else, // else
    While, // while
    Do, // do
    For, // for
    To, // to
    Break, // break
    Nil, // nil
    Symbol(String), // someVar
    Str(String), // "something"
    Number(i64), // 1337
    OpenComen, // /*
    CloseComen, // */
    LineComen, // //
    Quote, // "
}

pub enum LexicalError {
    LexError,
}

fn get_token(string_token: String) -> Option<Tok> {
    if Regex::new(r"^\.$").unwrap().is_match(&string_token) {
        Some(Tok::Point)
    } else if Regex::new(r"^:$").unwrap().is_match(&string_token) {
        Some(Tok::Colon)
    } else if Regex::new(r"^:=$").unwrap().is_match(&string_token) {
        Some(Tok::Assign)
    } else if Regex::new(r"^,$").unwrap().is_match(&string_token) {
        Some(Tok::Comma)
    } else if Regex::new(r"^;$").unwrap().is_match(&string_token) {
        Some(Tok::Semicolon)
    } else if Regex::new(r"^\($").unwrap().is_match(&string_token) {
        Some(Tok::OpenParen)
    } else if Regex::new(r"^\)$").unwrap().is_match(&string_token) {
        Some(Tok::CloseParen)
    } else if Regex::new(r"^\[$").unwrap().is_match(&string_token) {
        Some(Tok::OpenBracket)
    } else if Regex::new(r"^\]$").unwrap().is_match(&string_token) {
        Some(Tok::CloseBracket)
    } else if Regex::new(r"^{$").unwrap().is_match(&string_token) {
        Some(Tok::OpenBraces)
    } else if Regex::new(r"^}$").unwrap().is_match(&string_token) {
        Some(Tok::CloseBraces)
    } else if Regex::new(r"^&$").unwrap().is_match(&string_token) {
        Some(Tok::Ampersand)
    } else if Regex::new(r"^\|$").unwrap().is_match(&string_token) {
        Some(Tok::Pipe)
    } else if Regex::new(r"^=$").unwrap().is_match(&string_token) {
        Some(Tok::Equals)
    } else if Regex::new(r"^<$").unwrap().is_match(&string_token) {
        Some(Tok::Lt)
    } else if Regex::new(r"^<=$").unwrap().is_match(&string_token) {
        Some(Tok::Lte)
    } else if Regex::new(r"^>$").unwrap().is_match(&string_token) {
        Some(Tok::Gt)
    } else if Regex::new(r"^>=$").unwrap().is_match(&string_token) {
        Some(Tok::Gte)
    } else if Regex::new(r"^<>$").unwrap().is_match(&string_token) {
        Some(Tok::Neq)
    } else if Regex::new(r"^\+$").unwrap().is_match(&string_token) {
        Some(Tok::Plus)
    } else if Regex::new(r"^\-$").unwrap().is_match(&string_token) {
        Some(Tok::Minus)
    } else if Regex::new(r"^\*$").unwrap().is_match(&string_token) {
        Some(Tok::Times)
    } else if Regex::new(r"^/$").unwrap().is_match(&string_token) {
        Some(Tok::Div)
    } else if Regex::new(r"^type$").unwrap().is_match(&string_token) {
        Some(Tok::Type)
    } else if Regex::new(r"^array$").unwrap().is_match(&string_token) {
        Some(Tok::Array)
    } else if Regex::new(r"^of$").unwrap().is_match(&string_token) {
        Some(Tok::Of)
    } else if Regex::new(r"^var$").unwrap().is_match(&string_token) {
        Some(Tok::Var)
    } else if Regex::new(r"^function$").unwrap().is_match(&string_token) {
        Some(Tok::Function)
    } else if Regex::new(r"^let$").unwrap().is_match(&string_token) {
        Some(Tok::Let)
    } else if Regex::new(r"^in$").unwrap().is_match(&string_token) {
        Some(Tok::In)
    } else if Regex::new(r"^end$").unwrap().is_match(&string_token) {
        Some(Tok::End)
    } else if Regex::new(r"^if$").unwrap().is_match(&string_token) {
        Some(Tok::If)
    } else if Regex::new(r"^then$").unwrap().is_match(&string_token) {
        Some(Tok::Then)
    } else if Regex::new(r"^else$").unwrap().is_match(&string_token) {
        Some(Tok::Else)
    } else if Regex::new(r"^while$").unwrap().is_match(&string_token) {
        Some(Tok::While)
    } else if Regex::new(r"^do$").unwrap().is_match(&string_token) {
        Some(Tok::Do)
    } else if Regex::new(r"^for$").unwrap().is_match(&string_token) {
        Some(Tok::For)
    } else if Regex::new(r"^to$").unwrap().is_match(&string_token) {
        Some(Tok::To)
    } else if Regex::new(r"^break$").unwrap().is_match(&string_token) {
        Some(Tok::Break)
    } else if Regex::new(r"^nil$").unwrap().is_match(&string_token) {
        Some(Tok::Nil)
    } else if Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap().is_match(&string_token) {
        Some(Tok::Symbol(string_token))
    } else if Regex::new(r"^[0-9]+$").unwrap().is_match(&string_token) {
        Some(Tok::Number(string_token.parse::<i64>().unwrap()))
    } else if Regex::new(r"^/\*$").unwrap().is_match(&string_token) {
        Some(Tok::OpenComen)
    } else if Regex::new(r"^\*/$").unwrap().is_match(&string_token) {
        Some(Tok::CloseComen)
    } else if Regex::new(r"^//$").unwrap().is_match(&string_token) {
        Some(Tok::LineComen)
    } else if Regex::new(r#"^"$"#).unwrap().is_match(&string_token) {
        Some(Tok::Quote)
    } else {
        None
    }
}

enum LexerState {
    LexingTokens,
    LexingLineComment,
    LexingBlockComment(i64),
    LexingString(String),
}

pub struct Lexer<'input> {
    lines: Lines<'input>,
    state: LexerState,
}

impl<'input> Lexer<'input> {
    pub fn new(input: String) -> Self {
        Lexer {
            lines: input.lines(),
            state: LexerState::LexingTokens,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lines.next() {

                _ => LexicalError::LexError
            };
        }
    }
}

struct LineLexer<'input> {
    words: SplitWhitespace<'input>,
}

impl<'input> LineLexer<'input> {
    pub fn new(input: String) -> Self {
        LineLexer {
            words: input.split_whitespace(),
        }
    }
}

impl<'input> Iterator for LineLexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.words.next() {
                Some(word) => LexicalError::LexError,
                _ => break,
            };
        }
    }
}
