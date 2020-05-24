#![allow(
    clippy::missing_docs_in_private_items,
    clippy::use_debug,
    clippy::print_stdout,
    clippy::needless_pass_by_value,
    clippy::enum_variant_names,
    clippy::cognitive_complexity,
    clippy::trivial_regex,
    clippy::single_match
)]

use regex::Regex;
use std::str::{Chars, Lines, SplitWhitespace};

/*
*   IMPORTANT NOTE:
*
*   We're not representing position currently, we *could* add that as token parameter,
*   and the parser should adapt to it easily.
*
*   Equally important, Spanned should represent:
*
*   (@L, tok, @R) where
*
*   @L is the byte in the string where the token started
*   tok is the yielded token
*   @R is the byte in the string where the token ended
*
*   We're only working with tok and hardcoding the positions to be @L = 0, @R = 1.
*   This *works* but we're probably messing up debug capabilities.
*   LALRPOP's documentation is unclear as to why @L and @R are needed.
*/

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    Point,          // .
    Colon,          // :
    Assign,         // :=
    Comma,          // ,
    Semicolon,      // ;
    OpenParen,      // (
    CloseParen,     // )
    OpenBracket,    // [
    CloseBracket,   // ]
    OpenBraces,     // {
    CloseBraces,    // }
    Ampersand,      // &
    Pipe,           // |
    Equals,         // =
    Lt,             // <
    Lte,            // <=
    Gt,             // >
    Gte,            // >=
    Neq,            // <>
    Plus,           // +
    Minus,          // -
    Times,          // *
    Div,            // /
    Type,           // type
    Array,          // array
    Of,             // of
    Var,            // var
    Function,       // function
    Let,            // let
    In,             // in
    End,            // end
    If,             // if
    Then,           // then
    Else,           // else
    While,          // while
    Do,             // do
    For,            // for
    To,             // to
    Break,          // break
    Nil,            // nil
    Symbol(String), // someVar
    Str(String),    // "something"
    Number(i32),    // 1337
    OpenComen,      // /*
    CloseComen,     // */
    LineComen,      // //
    Quote,          // "
}

#[derive(Debug)]
pub enum LexicalError {
    LexError,
}

type TokenGeneratorFunction = Box<dyn (Fn(String) -> Tok) + Sync>;

lazy_static! {
    static ref REGEXES: Vec<(Regex, TokenGeneratorFunction)> = vec![
        (Regex::new(r"^\.$").unwrap(), Box::new(|_| Tok::Point)),
        (Regex::new(r"^:$").unwrap(), Box::new(|_| Tok::Colon)),
        (Regex::new(r"^:=$").unwrap(), Box::new(|_| Tok::Assign)),
        (Regex::new(r"^,$").unwrap(), Box::new(|_| Tok::Comma)),
        (Regex::new(r"^;$").unwrap(), Box::new(|_| Tok::Semicolon)),
        (Regex::new(r"^\($").unwrap(), Box::new(|_| Tok::OpenParen)),
        (Regex::new(r"^\)$").unwrap(), Box::new(|_| Tok::CloseParen)),
        (Regex::new(r"^\[$").unwrap(), Box::new(|_| Tok::OpenBracket)),
        (Regex::new(r"^\]$").unwrap(), Box::new(|_| Tok::CloseBracket)),
        (Regex::new(r"^\{$").unwrap(), Box::new(|_| Tok::OpenBraces)),
        (Regex::new(r"^}$").unwrap(), Box::new(|_| Tok::CloseBraces)),
        (Regex::new(r"^&$").unwrap(), Box::new(|_| Tok::Ampersand)),
        (Regex::new(r"^\|$").unwrap(), Box::new(|_| Tok::Pipe)),
        (Regex::new(r"^=$").unwrap(), Box::new(|_| Tok::Equals)),
        (Regex::new(r"^<$").unwrap(), Box::new(|_| Tok::Lt)),
        (Regex::new(r"^<=$").unwrap(), Box::new(|_| Tok::Lte)),
        (Regex::new(r"^>$").unwrap(), Box::new(|_| Tok::Gt)),
        (Regex::new(r"^>=$").unwrap(), Box::new(|_| Tok::Gte)),
        (Regex::new(r"^<>$").unwrap(), Box::new(|_| Tok::Neq)),
        (Regex::new(r"^\+$").unwrap(), Box::new(|_| Tok::Plus)),
        (Regex::new(r"^\-$").unwrap(), Box::new(|_| Tok::Minus)),
        (Regex::new(r"^\*$").unwrap(), Box::new(|_| Tok::Times)),
        (Regex::new(r"^/$").unwrap(), Box::new(|_| Tok::Div)),
        (Regex::new(r"^type$").unwrap(), Box::new(|_| Tok::Type)),
        (Regex::new(r"^array$").unwrap(), Box::new(|_| Tok::Array)),
        (Regex::new(r"^of$").unwrap(), Box::new(|_| Tok::Of)),
        (Regex::new(r"^var$").unwrap(), Box::new(|_| Tok::Var)),
        (Regex::new(r"^function$").unwrap(), Box::new(|_| Tok::Function)),
        (Regex::new(r"^let$").unwrap(), Box::new(|_| Tok::Let)),
        (Regex::new(r"^in$").unwrap(), Box::new(|_| Tok::In)),
        (Regex::new(r"^end$").unwrap(), Box::new(|_| Tok::End)),
        (Regex::new(r"^if$").unwrap(), Box::new(|_| Tok::If)),
        (Regex::new(r"^then$").unwrap(), Box::new(|_| Tok::Then)),
        (Regex::new(r"^else$").unwrap(), Box::new(|_| Tok::Else)),
        (Regex::new(r"^while$").unwrap(), Box::new(|_| Tok::While)),
        (Regex::new(r"^do$").unwrap(), Box::new(|_| Tok::Do)),
        (Regex::new(r"^for$").unwrap(), Box::new(|_| Tok::For)),
        (Regex::new(r"^to$").unwrap(), Box::new(|_| Tok::To)),
        (Regex::new(r"^break$").unwrap(), Box::new(|_| Tok::Break)),
        (Regex::new(r"^nil$").unwrap(), Box::new(|_| Tok::Nil)),
        (Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap(), Box::new(Tok::Symbol)),
        (Regex::new(r"^[0-9]+$").unwrap(), Box::new(|string_token| Tok::Number(string_token.parse::<i32>().unwrap()))),
        (Regex::new(r"^/\*$").unwrap(), Box::new(|_| Tok::OpenComen)),
        (Regex::new(r"^\*/$").unwrap(), Box::new(|_| Tok::CloseComen)),
        (Regex::new(r"^//$").unwrap(), Box::new(|_| Tok::LineComen)),
        (Regex::new(r#"^"$"#).unwrap(), Box::new(|_| Tok::Quote)),
    ];
}

fn get_token(string_token: String) -> Option<Tok> {
    use rayon::prelude::*;
    Some(REGEXES
        .par_iter()
        .map(|(r, t)| {
            if r.is_match(&string_token) {
                Some(t)
            } else {
                None
            }
        })
        .find_first(|t| t.is_some())??(string_token))
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum LexerState {
    LexingTokens,
    LexingLineComment,
    LexingBlockComment(i32),
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
    current_string: String,
}

impl<'input> Lexer<'input> {
    pub fn new(mut line_iterator: Lines<'input>) -> Self {
        let current: &'input str = line_iterator.next().unwrap_or_else(|| "");

        Lexer {
            lines: line_iterator,
            line_lexer: LineLexer::new(current.split_whitespace(), LexerState::LexingTokens),
            state: LexerState::LexingTokens,
            current_string: String::from(current),
        }
    }

    pub fn transition(&mut self, new_state: LexerState) {
        self.state = new_state.clone();
        self.line_lexer.transition(new_state);
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.line_lexer.next() {
                Some(Ok(Consumpion {
                    token: Some(t),
                    state_transition,
                })) => {
                    // We got a token and possibly a state transition!
                    match t {
                        Tok::OpenComen => {
                            let new_state: LexerState = match self.state {
                                LexerState::LexingBlockComment(n) => {
                                    LexerState::LexingBlockComment(n + 1)
                                }
                                _ => LexerState::LexingBlockComment(1),
                            };

                            self.transition(new_state);
                            continue;
                        }
                        Tok::CloseComen => {
                            match self.state {
                                LexerState::LexingBlockComment(1) => {
                                    self.transition(LexerState::LexingTokens);
                                    continue;
                                }
                                LexerState::LexingBlockComment(n) => {
                                    self.transition(LexerState::LexingBlockComment(n - 1));
                                    continue;
                                }
                                _ => {
                                    return Some(Err(LexicalError::LexError));
                                }
                            };
                        }
                        Tok::Quote => {
                            match self.state.clone() {
                                LexerState::LexingString(s) => {
                                    self.transition(LexerState::LexingTokens);
                                    return Some(Ok((0, Tok::Str(s), 1)));
                                }
                                _ => self.transition(LexerState::LexingString(String::from(""))),
                            };

                            continue;
                        }
                        _ => {
                            self.transition(state_transition);
                            return Some(Ok((0, t, 1)));
                        }
                    };
                }
                Some(Ok(Consumpion {
                    token: None,
                    state_transition,
                })) => {
                    // We just got a state transition!
                    self.transition(state_transition);
                    continue;
                }
                Some(Err(e)) => {
                    return Some(Err(e));
                }
                None => {

                    if let LexerState::LexingString(s) = self.state.clone() {
                        let mut new_string = s.clone();
                        new_string.push_str(&self.current_string);
                        new_string.push('\n');

                        self.transition(LexerState::LexingString(new_string));
                    }
                    // Finished lexing this line!
                    match self.lines.next() {
                        Some(line) => {
                            // Handle state changes, if we're Lexing a line comment, go back to token mode. If not, keep the state.
                            let new_state = if self.state == LexerState::LexingLineComment {
                                LexerState::LexingTokens
                            } else {
                                self.state.clone()
                            };

                            self.transition(new_state);

                            // Create the lexer for the next line
                            self.current_string = String::from(line);
                            self.line_lexer =
                                LineLexer::new(line.split_whitespace(), self.state.clone());

                            // Go keep consuming lines...
                            continue;
                        }
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
    current_word: &'input str,
    state: LexerState,
}

impl<'input> LineLexer<'input> {
    pub fn new(mut words_iter: SplitWhitespace<'input>, state: LexerState) -> Self {
        let current: &'input str = words_iter.next().unwrap_or_else(|| "");
        let initial_state: LexerState = state.clone();

        LineLexer {
            words: words_iter,
            current_word: current,
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

// Returns the location of the REST of the string (past the first unescaped quote)
// -1 if no matches.
fn quote_location(s: &str) -> i32 {
    let re = Regex::new(r#"(^"|(?:[^\\])")"#).unwrap();
    match re.find(s) {
        Some(mat) => mat.end() as i32,
        _ => -1,
    }
}

impl<'input> Iterator for LineLexer<'input> {
    type Item = Result<Consumpion, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == LexerState::LexingLineComment {
            return None;
        }

        loop {
            if self.state == LexerState::LexingTokens {
                if let Some(tok) = get_token(String::from(self.current_word)) {
                    // We should check if this token is yieldable! Probably we should just transition
                    // For now we'll just yield and keep the state
                    self.current_word = self.words.next().unwrap_or_else(|| "");
                    self.char_lexer = CharLexer::new(self.current_word.chars(), self.state.clone());

                    // If the token is // just transition to LexingLineComment
                    if tok == Tok::LineComen {
                        return Some(Ok(Consumpion::new(None, LexerState::LexingLineComment)));
                    }

                    return Some(Ok(Consumpion::new(Some(tok), self.state.clone())));
                }
            }

            if let LexerState::LexingString(s) = self.state.clone() {
                let quote_loc = quote_location(self.current_word);

                if quote_loc > 0 {
                    // Found an unescaped quote!
                    let position: usize = quote_loc as usize;

                    // This line, until the quote
                    let string_part = &self.current_word[..(position - 1)];

                    // From the quote to the end
                    self.current_word = &self.current_word[position..];
                    let rest = &self.current_word;

                    let mut full_string = s;
                    full_string.push_str(string_part);

                    self.char_lexer = CharLexer::new(rest.chars(), self.state.clone());
                    return Some(Ok(Consumpion::new(
                        Some(Tok::Str(full_string)),
                        LexerState::LexingTokens,
                    )));
                } else {
                    // No unescaped quote...
                    let mut new_string = s.clone();
                    new_string.push_str(self.current_word);
                    new_string.push(' ');
                    self.transition(LexerState::LexingString(new_string));

                    if let Some(word) = self.words.next() {
                        self.current_word = word;
                        self.char_lexer = CharLexer::new(word.chars(), self.state.clone());
                        continue;
                    } else {
                        return None;
                    }
                }
            }

            match self.char_lexer.next() {
                Some(Ok(Consumpion {
                    token,
                    state_transition,
                })) => {
                    // We don't need to transition explicitly, just lift the new state.
                    if token == Some(Tok::Quote) {
                        // We'll transition into LexingString, crop the current_word to the first quote.
                        let quote_loc = quote_location(self.current_word) as usize;
                        self.current_word = &self.current_word[quote_loc..];
                    } else if token == Some(Tok::CloseComen) {
                        self.current_word = "";
                    }
                    return Some(Ok(Consumpion::new(token, state_transition)));
                }
                Some(Err(e)) => return Some(Err(e)),
                None => {
                    if let Some(word) = self.words.next() {
                        // No more tokens to get from this word, move to the next one
                        self.current_word = word;
                        self.char_lexer = CharLexer::new(word.chars(), self.state.clone());
                        continue;
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}

/* Try lexing tokens from each individual char */
struct CharLexer<'input> {
    chars: Chars<'input>,
    current_string: String,
    state: LexerState,
}

impl<'input> CharLexer<'input> {
    pub fn new(chars_iter: Chars<'input>, state: LexerState) -> Self {
        CharLexer {
            chars: chars_iter,
            current_string: String::from(""),
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
        loop {
            if let Some(current_char) = self.chars.next() {
                // There are more chars, are we still building a token?
                let mut new_string: String = self.current_string.clone();
                new_string.push(current_char);

                if let Some(_token) = get_token(new_string.clone()) {
                    // Yeah, we're still building...
                    self.current_string = new_string;
                    continue;
                } else {
                    // NO! nice, yield this token (if yieldable) and transition if needed.
                    if let Some(prev_token) = get_token(self.current_string.clone()) {
                        self.current_string = String::from("");
                        self.current_string.push(current_char);

                        match (self.state.clone(), prev_token.clone()) {
                            (LexerState::LexingBlockComment(_), Tok::OpenComen)
                            | (LexerState::LexingBlockComment(_), Tok::CloseComen) => {
                                self.current_string = String::from("");
                                return Some(Ok(Consumpion::new(
                                    Some(prev_token),
                                    self.state.clone(),
                                )));
                            }
                            (LexerState::LexingBlockComment(_), _) => {
                                continue;
                            }
                            _ => {
                                // If the token is // just transition to LexingLineComment
                                if prev_token == Tok::LineComen {
                                    return Some(Ok(Consumpion::new(
                                        None,
                                        LexerState::LexingLineComment,
                                    )));
                                }

                                return Some(Ok(Consumpion::new(
                                    Some(prev_token),
                                    self.state.clone(),
                                )));
                            }
                        }
                    } else {
                        match self.state {
                            // If we're lexing comments, no problem
                            LexerState::LexingBlockComment(_) => return None,

                            // If we're lexing tokens something's wrong!
                            _ => return Some(Err(LexicalError::LexError)),
                        }
                    }
                }
            } else if self.current_string.is_empty() {
                // No chars and current_string is empty, nothing to yield here, we're done.
                return None;
            } else if let Some(prev_token) = get_token(self.current_string.clone()) {
                // No more chars, but there's a current_string. We should have a token to yield.

                match (self.state.clone(), prev_token.clone()) {
                    (LexerState::LexingBlockComment(_), Tok::OpenComen)
                    | (LexerState::LexingBlockComment(_), Tok::CloseComen) => {
                        self.current_string = String::from("");
                        return Some(Ok(Consumpion::new(Some(prev_token), self.state.clone())));
                    }
                    (LexerState::LexingBlockComment(_), _) => {
                        self.current_string = String::from("");
                        return None;
                    }
                    _ => {
                        self.current_string = String::from("");
                        return Some(Ok(Consumpion::new(Some(prev_token), self.state.clone())));
                    }
                }
            } else {
                match self.state {
                    // If we're lexing comments, no problem
                    LexerState::LexingBlockComment(_) => return None,

                    // If we're lexing tokens something's wrong!
                    _ => return Some(Err(LexicalError::LexError)),
                }
            }
        }
    }
}
