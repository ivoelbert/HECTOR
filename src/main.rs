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
)]

#[allow(dead_code)]
mod ast;
#[allow(dead_code)] // Estos dead_code hay que sacarlos cuando terminemos estos modulos
mod seman;
#[cfg(test)]
mod test;

use ast::tigerabs::*;
use ast::position::{Pos};


use seman::tigerseman::*;

extern crate lalrpop_util;
extern crate pathfinding;

fn possed_exp(exp: _Exp) -> Exp {
    Exp {node: exp, pos: Pos {line: 0, column: 0}}
}

fn boxed_exp(exp: _Exp) -> Box<Exp> {
    Box::new(Exp {node: exp, pos: Pos {line: 0, column: 0}})
}
fn main() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("List"),
                    Ty::Record(vec![
                        Field {
                            name: Symbol::from("head"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        },
                        Field {
                            name: Symbol::from("tail"),
                            typ: Ty::Name(Symbol::from("List")),
                            escape: false,
                        }
                    ])
                ),
                Pos{line: 0, column: 1}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("List")),
                    boxed_exp(_Exp::RecordExp {
                        fields: vec![
                            (Symbol::from("head"), boxed_exp(_Exp::IntExp(1))),
                            (Symbol::from("tail"), boxed_exp(_Exp::NilExp))
                        ],
                        typ: Symbol::from("List"),
                    })
                ),
                Pos{line: 0, column: 2}
            )],
        body: boxed_exp(_Exp::VarExp(
            Var::FieldVar(
                Box::new(Var::SimpleVar(Symbol::from("foo"))),
                Symbol::from("head")
            )
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}
