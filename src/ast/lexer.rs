
use regex::Regex;
use std::str::CharIndices;

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
    Str(String), // "something"
    Number(i64), // 1337
}

enum InternalTok {
    Public(Tok),
    OpenComen, // /*
    CloseComen, // */
    LineComen, // //
    Quote, // "
}

pub enum LexicalError {
    LexError,
}

/*
*   STATE MACHINE:
*   Lexer can be in 3 possible states:
*   1- GeneratingToken(current_raw_token: string)
*   2- GeneratingString(last_char: char, current_string: string) keeps the last char to build escaped quotes
*   3- ConsumingBlockComment(last_char: char, level: number) keeps the last char to build comment tokens
*   4- ConsumingLineComment
*/

pub enum State {
    GeneratingToken(String),
    GeneratingString(char, String),
    ConsumingBlockComment(char, i64),
    ConsumingLineComment,
}

struct CharConsumption {
    token: Option<InternalTok>,
    state_transition: State,
}


fn process_character(c: char, state: State) -> Result<CharConsumption, LexicalError> {
    match state {
        State::GeneratingToken(current_tok) => {
            generate_token(c, current_tok)
        },
        State::GeneratingString(last_char, current_string) => {
            generate_string(c, last_char, current_string)
        },
        State::ConsumingBlockComment(last_char, level) => {
            consume_block_comment(c, last_char, level)
        },
        State::ConsumingLineComment => {
            consume_line_comment(c)
        },
    }
}

fn generate_token(c: char, current_tok: String) -> Result<CharConsumption, LexicalError> {
    match c {
        ' ' | '\t' => {
            Ok(CharConsumption {
                token: get_token(current_tok),
                state_transition: State::GeneratingToken(String::from("")),
            })
        },
        '\n' => {
            Ok(CharConsumption {
                token: get_token(current_tok),
                state_transition: State::GeneratingToken(String::from("")),
            })
        },
        _ => {
            let mut new_current_tok: String = current_tok.clone();
            new_current_tok.push(c);

            let parsed_token: Option<InternalTok> = get_token(new_current_tok);

            match parsed_token {
                Some(InternalTok::LineComen) => {
                    Ok(CharConsumption {
                        token: None,
                        state_transition: State::ConsumingLineComment,
                    })
                },
                Some(InternalTok::OpenComen) => {
                    Ok(CharConsumption {
                        token: None,
                        state_transition: State::ConsumingBlockComment(' ', 1),
                    })
                },
                Some(InternalTok::CloseComen) => {
                    Err(LexicalError::LexError)
                },
                Some(InternalTok::Quote) => {
                    Ok(CharConsumption {
                        token: None,
                        state_transition: State::GeneratingString(' ', String::from("")),
                    })
                },
                Some(_) => {
                    Ok(CharConsumption {
                        token: None,
                        state_transition: State::GeneratingToken(new_current_tok),
                    })
                }
                None => {
                    let mut starting_raw_tok: String = String::from("");
                    starting_raw_tok.push(c);

                    Ok(CharConsumption {
                        token: get_token(current_tok),
                        state_transition: State::GeneratingToken(starting_raw_tok),
                    })
                }
            }
        },
    }
}

fn generate_string(c: char, last_char: char, mut current_string: String) -> Result<CharConsumption, LexicalError> {
    match (last_char, c) {
        (_ , '"') => {
            Ok(CharConsumption {
                token: Some(InternalTok::Public(Tok::Str(current_string))),
                state_transition: State::GeneratingToken(String::from("")),
            })
        },
        _ => {
            current_string.push(c);

            Ok(CharConsumption {
                token: None,
                state_transition: State::GeneratingString(c, current_string),
            })
        }
    }
}

fn consume_block_comment(c: char, last_char: char, level: i64) -> Result<CharConsumption, LexicalError> {
    match (last_char, c, level) {

        // Try to parse closing block
        ('*', '/', 1) => {
            Ok(CharConsumption {
                token: None,
                state_transition: State::GeneratingToken(String::from("")),
            })
        },
        ('*', '/', n) => {
            Ok(CharConsumption {
                token: None,
                state_transition: State::ConsumingBlockComment('/', n - 1),
            })
        },

        // Try to parse open block
        ('/', '*', n) => {
            Ok(CharConsumption {
                token: None,
                state_transition: State::ConsumingBlockComment('/', n + 1),
            })
        },

        // Ignore other characters
        _ => {
            Ok(CharConsumption {
                token: None,
                state_transition: State::ConsumingBlockComment(c, level),
            })
        }
    }
}

fn consume_line_comment(c: char) -> Result<CharConsumption, LexicalError> {
    match c {
        '\n' => {
            Ok(CharConsumption {
                token: None,
                state_transition: State::GeneratingToken(String::from("")),
            })
        },
        _ => {
            Ok(CharConsumption {
                token: None,
                state_transition: State::ConsumingLineComment,
            })
        }
    }
}


fn get_token(string_token: String) -> Option<InternalTok> {
    if Regex::new(r"^\.$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Point))
    } else if Regex::new(r"^:$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Colon))
    } else if Regex::new(r"^:=$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Assign))
    } else if Regex::new(r"^,$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Comma))
    } else if Regex::new(r"^;$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Semicolon))
    } else if Regex::new(r"^\($").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::OpenParen))
    } else if Regex::new(r"^\)$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::CloseParen))
    } else if Regex::new(r"^\[$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::OpenBracket))
    } else if Regex::new(r"^\]$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::CloseBracket))
    } else if Regex::new(r"^{$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::OpenBraces))
    } else if Regex::new(r"^}$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::CloseBraces))
    } else if Regex::new(r"^&$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Ampersand))
    } else if Regex::new(r"^\|$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Pipe))
    } else if Regex::new(r"^=$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Equals))
    } else if Regex::new(r"^<$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Lt))
    } else if Regex::new(r"^<=$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Lte))
    } else if Regex::new(r"^>$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Gt))
    } else if Regex::new(r"^>=$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Gte))
    } else if Regex::new(r"^<>$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Neq))
    } else if Regex::new(r"^\+$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Plus))
    } else if Regex::new(r"^\-$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Minus))
    } else if Regex::new(r"^\*$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Times))
    } else if Regex::new(r"^/$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Div))
    } else if Regex::new(r"^type$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Type))
    } else if Regex::new(r"^array$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Array))
    } else if Regex::new(r"^of$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Of))
    } else if Regex::new(r"^var$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Var))
    } else if Regex::new(r"^function$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Function))
    } else if Regex::new(r"^let$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Let))
    } else if Regex::new(r"^in$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::In))
    } else if Regex::new(r"^end$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::End))
    } else if Regex::new(r"^if$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::If))
    } else if Regex::new(r"^then$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Then))
    } else if Regex::new(r"^else$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Else))
    } else if Regex::new(r"^while$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::While))
    } else if Regex::new(r"^do$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Do))
    } else if Regex::new(r"^for$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::For))
    } else if Regex::new(r"^to$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::To))
    } else if Regex::new(r"^break$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Break))
    } else if Regex::new(r"^nil$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Nil))
    } else if Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Str(string_token)))
    } else if Regex::new(r"^[0-9]+$").unwrap().is_match(&string_token) {
        Some(InternalTok::Public(Tok::Number(string_token.parse::<i64>().unwrap())))
    } else if Regex::new(r"^/\*$").unwrap().is_match(&string_token) {
        Some(InternalTok::OpenComen)
    } else if Regex::new(r"^\*/$").unwrap().is_match(&string_token) {
        Some(InternalTok::CloseComen)
    } else if Regex::new(r"^//$").unwrap().is_match(&string_token) {
        Some(InternalTok::LineComen)
    } else if Regex::new(r#"^"$"#).unwrap().is_match(&string_token) {
        Some(InternalTok::Quote)
    } else {
        None
    }
}










#[derive(Clone)]
pub struct RawTok<'input> {
    start: i32,
    end: i32,
    col: i32,
    line: i32,
    tok: &'input str,
}

impl<'input> RawTok<'input> {
    pub fn get_token(&mut self) -> Option<InternalTok> {
        if Regex::new(r"^\.$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Point))
        } else if Regex::new(r"^:$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Colon))
        } else if Regex::new(r"^:=$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Assign))
        } else if Regex::new(r"^,$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Comma))
        } else if Regex::new(r"^;$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Semicolon))
        } else if Regex::new(r"^\($").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::OpenParen))
        } else if Regex::new(r"^\)$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::CloseParen))
        } else if Regex::new(r"^\[$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::OpenBracket))
        } else if Regex::new(r"^\]$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::CloseBracket))
        } else if Regex::new(r"^{$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::OpenBraces))
        } else if Regex::new(r"^}$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::CloseBraces))
        } else if Regex::new(r"^&$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Ampersand))
        } else if Regex::new(r"^\|$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Pipe))
        } else if Regex::new(r"^=$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Equals))
        } else if Regex::new(r"^<$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Lt))
        } else if Regex::new(r"^<=$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Lte))
        } else if Regex::new(r"^>$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Gt))
        } else if Regex::new(r"^>=$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Gte))
        } else if Regex::new(r"^<>$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Neq))
        } else if Regex::new(r"^\+$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Plus))
        } else if Regex::new(r"^\-$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Minus))
        } else if Regex::new(r"^\*$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Times))
        } else if Regex::new(r"^/$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Div))
        } else if Regex::new(r"^type$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Type))
        } else if Regex::new(r"^array$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Array))
        } else if Regex::new(r"^of$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Of))
        } else if Regex::new(r"^var$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Var))
        } else if Regex::new(r"^function$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Function))
        } else if Regex::new(r"^let$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Let))
        } else if Regex::new(r"^in$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::In))
        } else if Regex::new(r"^end$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::End))
        } else if Regex::new(r"^if$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::If))
        } else if Regex::new(r"^then$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Then))
        } else if Regex::new(r"^else$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Else))
        } else if Regex::new(r"^while$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::While))
        } else if Regex::new(r"^do$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Do))
        } else if Regex::new(r"^for$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::For))
        } else if Regex::new(r"^to$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::To))
        } else if Regex::new(r"^break$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Break))
        } else if Regex::new(r"^nil$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Nil))
        } else if Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Str(self.tok.to_string())))
        } else if Regex::new(r"^[0-9]+$").unwrap().is_match(self.tok) {
            Some(InternalTok::Public(Tok::Number(self.tok.parse::<i64>().unwrap())))
        } else if Regex::new(r"^/\*$").unwrap().is_match(self.tok) {
            Some(InternalTok::OpenComen)
        } else if Regex::new(r"^\*/$").unwrap().is_match(self.tok) {
            Some(InternalTok::CloseComen)
        } else if Regex::new(r"^//$").unwrap().is_match(self.tok) {
            Some(InternalTok::LineComen)
        } else if Regex::new(r#"^"$"#).unwrap().is_match(self.tok) {
            Some(InternalTok::Quote)
        } else {
            None
        }
    }
}

pub struct RawTokens<'input> {
    chars: CharIndices<'input>,
    current_raw_tok: Option<RawTok<'input>>,
    current_col: i32,
    current_line: i32,
}

impl<'input> RawTokens<'input> {
    pub fn new(input: &'input str) -> Self {
        RawTokens {
            chars: input.char_indices(),
            current_raw_tok: None,
            current_col: 0,
            current_line: 0,
        }
    }

    fn handle_white_space(&mut self) -> Option<RawTok<'input>> {
        self.current_col += 1;

        match &self.current_raw_tok {
            Some(rawtok) => {
                let return_token: RawTok = rawtok.clone();

                self.current_raw_tok = None;

                Some(return_token)
            }
            None => None,
        }
    }

    fn handle_new_line(&mut self, new_line: bool) -> Option<RawTok<'input>> {
        if new_line {
            self.current_col = 0;
            self.current_line += 1;
        } else {
            self.current_col += 1;
        }

        match &self.current_raw_tok {
            Some(rawtok) => {
                let return_token = rawtok.clone();

                self.current_raw_tok = None;

                Some(return_token)
            }
            None => None,
        }
    }
}

impl<'input> Iterator for RawTokens<'input> {
    type Item = RawTok<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_i, ' ')) | Some((_i, '\t')) => match self.handle_new_line(false) {
                    Some(tok) => return Some(tok),
                    None => continue,
                },
                Some((_i, '\n')) => match self.handle_new_line(true) {
                    Some(tok) => return Some(tok),
                    None => continue,
                },
                Some((_i, c)) => {
                    /*
                     *   Should check if this character extends the current token to a larger one.
                     *   In that case we continue building with this character.
                     *   Else, we return the current token and start a new one from here.
                     */

                    continue;
                }
                None => return self.handle_new_line(false),
            }
        }
    }
}

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, ' ')) => return Some(Ok((i, Tok::Space, i + 1))),
                Some((i, '\t')) => return Some(Ok((i, Tok::Tab, i + 1))),
                Some((i, '\n')) => return Some(Ok((i, Tok::Linefeed, i + 1))),

                None => return None, // End of file
                _ => continue,       // Comment; skip this character
            }
        }
    }
}