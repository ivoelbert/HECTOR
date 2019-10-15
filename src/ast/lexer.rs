
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::use_debug,
    clippy::print_stdout,
    clippy::needless_pass_by_value,
    clippy::enum_variant_names,
    clippy::cognitive_complexity,
    clippy::trivial_regex,
)]

use regex::Regex;
use std::str::{Lines, SplitWhitespace, Chars};

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
    } else if Regex::new(r"^\{$").unwrap().is_match(&string_token) {
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

#[derive(PartialEq, Eq, Clone)]
pub enum LexerState {
    LexingTokens,
    LexingLineComment,
    LexingBlockComment(i64),
    LexingString(String),
}

struct Consumpion {
    token: Option<Tok>,
    state_transition: LexerState,
}

impl Consumpion {
    pub fn new(t: Option<Tok>, new_state: LexerState) -> Self {
        Consumpion {
            token: t,
            state_transition: new_state,
        }
    }
}

/* Lex a program */
pub struct Lexer<'input> {
    lines: Lines<'input>,
    line_lexer: LineLexer<'input>,
    state: LexerState,
}

impl<'input> Lexer<'input> {
    pub fn new(mut line_iterator: Lines<'input>) -> Self {
        let current: &'input str = line_iterator.next().unwrap_or_else(|| "");

        Lexer {
            lines: line_iterator,
            line_lexer: LineLexer::new(current.split_whitespace(), LexerState::LexingTokens),
            state: LexerState::LexingTokens,
        }
    }

    pub fn transition(&mut self, new_state: LexerState) {
        self.state = new_state.clone();
        self.line_lexer.transition(new_state.clone());
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Tok, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.line_lexer.next() {
                Some(Ok(Consumpion { token: Some(t), state_transition })) => {
                    // Remember to transition before returning that token!
                    self.transition(state_transition);
                    return Some(Ok(t))
                },
                Some(Ok(Consumpion { token: None, state_transition })) => {
                    // Remember to transition, but keep on lexing lines, nothing to yield.
                    self.state = state_transition;
                    continue;
                },
                Some(Err(e)) => {
                    return Some(Err(e))
                },
                None => {
                    // Finished lexing this line!
                    match self.lines.next() {
                        Some(line) => {
                            // Handle state changes, if we're Lexing a line comment, go back to token mode. If not, keep the state.
                            self.state = if self.state == LexerState::LexingLineComment { LexerState::LexingTokens } else { self.state.clone() };

                            // Create the lexer for the next line
                            self.line_lexer = LineLexer::new(line.split_whitespace(), self.state.clone());

                            // Go keep consuming lines...
                            continue;
                        },
                        None => return None,
                    }
                }
            }
        }
    }
}

/* Lex each individual line */
struct LineLexer<'input> {
    words: SplitWhitespace<'input>,
    char_lexer: CharLexer<'input>,
    state: LexerState,
}

impl<'input> LineLexer<'input> {
    pub fn new(mut words_iter: SplitWhitespace<'input>, state: LexerState) -> Self {
        let current: &'input str = words_iter.next().unwrap_or_else(|| "");
        let initial_state: LexerState = state.clone();

        LineLexer {
            words: words_iter,
            char_lexer: CharLexer::new(current.chars(), state),
            state: initial_state,
        }
    }

    pub fn transition(&mut self, new_state: LexerState) {
        let char_lexer_state: LexerState = new_state.clone();

        self.state = new_state;
        self.char_lexer.transition(char_lexer_state);
    }
}

impl<'input> Iterator for LineLexer<'input> {
    type Item = Result<Consumpion, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.words.next() {
            Some(_word) => {
                Some(Err(LexicalError::LexError))
            },
            _ => None,
        }
    }
}

/* Try lexing tokens from each individual char */
struct CharLexer<'input> {
    chars: Chars<'input>,
    state: LexerState,
}

impl<'input> CharLexer<'input> {
    pub fn new(chars_iter: Chars<'input>, state: LexerState) -> Self {
        CharLexer {
            chars: chars_iter,
            state,
        }
    }

    pub fn transition(&mut self, new_state: LexerState) {
        self.state = new_state;
    }
}

impl<'input> Iterator for CharLexer<'input> {
    type Item = Result<Consumpion, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(_word) => Some(Err(LexicalError::LexError)),
            _ => None,
        }
    }
}
