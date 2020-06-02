#![allow(
    clippy::clone_on_copy,
    clippy::double_parens,
    clippy::needless_return,
    clippy::redundant_static_lifetimes,
    clippy::redundant_closure,
    unused_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    // clippy::missing_docs_in_private_items,
)]
//! Tiger parser
//! This module is later extended with lalrpop
use super::{AST};
use super::position::{Pos};
use super::lexer::Lexer;
use lalrpop_util::lalrpop_mod;
use serde::{Serialize};

#[macro_use]
lalrpop_mod!(pub parser);

#[derive(Debug, Serialize)]
/// An error ocurred while parsing
pub enum ParseError {
    /// Unexpected token
    UnexpectedToken(Pos),
}

/// Transform Tiger source code into an AST
/// All nodes have type = untyped
pub fn parse(str_src : &str) -> Result<AST, ParseError> {
    let lexed = Lexer::new(str_src.lines());

    match parser::ExprParser::new().parse(lexed) {
        Ok(box_exp) => Ok(*box_exp),
        Err(..) => {
            Err(ParseError::UnexpectedToken(Pos {column: 0, line: 0}))
        }
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    use super::super::{Exp, Var, VarKind, Oper};
    #[test]
    #[wasm_bindgen_test]
    fn number() {
        let input = String::from("0");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Int(0),
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn string() {
        let input = String::from("\"perro\"");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::String(_),
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn breakexp() {
        let input = String::from("break");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Break,
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn simplevar() {
        let input = String::from("foo");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Var(Var{kind: VarKind::Simple(_), ..}),
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn subscriptvar() {
        let input = String::from("foo[0]");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Var(Var{kind: VarKind::Subscript(..), ..}),
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn fieldvar() {
        let input = String::from("foo.baz");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Var(Var{kind: VarKind::Field(..), ..}),
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn callexp() {
        let input = String::from("foo(1, \"perro\", baz)");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Call {..},
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn custom() {
        let input = String::from("(2 * 2 = 2 + 2) & 2 - 2 <> 2 + 2");
        let parsed = parse(&input);

        match parsed {
            Ok(exp) => println!("Parsed expresion:\n\n{:?}\n\n", exp),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn simple_sum() {
        let input = String::from("2 + 2");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Op {
                        oper: Oper::Plus,
                        ..
                    },
                pos: Pos { line: 0, column: 0 },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn simple_mult() {
        let input = String::from("2 * 2");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Op {
                        oper: Oper::Times,
                        ..
                    },
                pos: Pos { line: 0, column: 0 },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn simple_comp() {
        let input = String::from("2 >= 2");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Op {
                        oper: Oper::Ge,
                        ..
                    },
                pos: Pos { line: 0, column: 0 },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn seqexp() {
        let input = String::from("(1;2)");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node: Exp::Seq(_),
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn assignexp() {
        let input = String::from("foo := 42");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Assign {
                        var: Var{kind: VarKind::Simple(_), ..},
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn ifexp() {
        let input = String::from("if 1 then 2 else 3");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::If {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn forexp() {
        let input = String::from("for i :=0 to 100 do 1");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::For {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec() {
        let input = String::from("let function foo() = 1 in 2 end");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Let {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }


    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_namety() {
        let input = String::from("let type numeritos = int in 2  end");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Let {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_recordty() {
        let input = String::from("let type name = {first_name: string, last_name: string} in 2  end");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Let {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_arrayty() {
        let input = String::from("let type intArray = array of int in 2  end");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Let {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }


    #[test]
    #[wasm_bindgen_test]
    fn letexp_arrayexp() {
        let input = String::from("arrtype [10] of 0");
        let parsed = parse(&input);
        match parsed {
            Ok(AST {
                node:
                    Exp::Array {
                        ..
                    },
                ..
            }) => (),
            Ok(..) => panic!("wrong parsing"),
            Err(..) => panic!("parser fails in a well-formed expression"),
        }
    }
}