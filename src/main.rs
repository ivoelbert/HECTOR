#![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
 )]
#![allow(
    clippy::missing_docs_in_private_items, // esto es una verdadera paja
    clippy::implicit_return, // se contradice con otro?
    clippy::use_debug, // para debuguear el parser
    clippy::print_stdout,
    clippy::needless_pass_by_value, // para tener los translate muertos
    dead_code,
)]

mod ast;
mod typecheck;
mod tree;

#[cfg(test)]
mod test;

use ast::*;
use tree::escape::find_escapes;
//extern crate lalrpop_util;
extern crate pathfinding;

fn possed_exp(exp: _Exp) -> Exp {
    Exp {node: exp, pos: Pos {line: 0, column: 0}}
}

fn boxed_exp(exp: _Exp) -> Box<Exp> {
    Box::new(Exp {node: exp, pos: Pos {line: 0, column: 0}})
}

fn main() {
    // let exp = possed_exp(_Exp::Let {
    //     decs: vec![
    //         Dec::TypeDec(vec![(
    //             _TypeDec::new(
    //                 Symbol::from("List"),
    //                 Ty::Record(vec![
    //                     Field {
    //                         name: Symbol::from("head"),
    //                         typ: Ty::Name(Symbol::from("int")),
    //                         escape: false,
    //                     },
    //                     Field {
    //                         name: Symbol::from("tail"),
    //                         typ: Ty::Name(Symbol::from("List")),
    //                         escape: false,
    //                     }
    //                 ])
    //             ),
    //             Pos{line: 0, column: 1}
    //         )]),
    //         Dec::VarDec(
    //             _VarDec::new(
    //                 Symbol::from("foo"),
    //                 Some(Symbol::from("List")),
    //                 boxed_exp(_Exp::Record {
    //                     fields: vec![
    //                         (Symbol::from("head"), boxed_exp(_Exp::Int(1))),
    //                         (Symbol::from("tail"), boxed_exp(_Exp::Nil))
    //                     ],
    //                     typ: Symbol::from("List"),
    //                 })
    //             ),
    //             Pos{line: 0, column: 2}
    //         )],
    //     body: boxed_exp(_Exp::Var(
    //         Var::Field(
    //             Box::new(Var::Simple(Symbol::from("foo"))),
    //             Symbol::from("head")
    //         )
    //     ))
    // });
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::VarDec(
                _VarDec{name: Symbol::from("var1"), escape: false, init: boxed_exp(_Exp::Int(1)), typ: None}, // var defined here
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Var(Var::Simple(Symbol::from("var1")))), // and used here
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![possed_exp(_Exp::Int(2))]
        })
    });
    let escaped_exp = find_escapes(exp);
    if let Exp {node: _Exp::Let {decs, ..}, ..} = escaped_exp {
        if let Some((Dec::VarDec(_VarDec{escape, ..}, ..), ..)) = decs.split_first() {
            if *escape {
                return () // PASS
            } else {
                panic!("wrong escape")
            }
        }
    }
    panic!("wrong structure")
}
