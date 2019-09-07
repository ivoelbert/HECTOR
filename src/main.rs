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
use ast::tigerabs::_Exp::{OpExp, IntExp};
use ast::position::{Pos, WithPos};


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
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("C"), Ty::Name(Symbol::from("B"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("B"), Ty::Name(Symbol::from("A"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("A"), Ty::Name(Symbol::from("int"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_exp(_Exp::UnitExp)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(..) => panic!("wrong type"),
        Err(..) => panic!("type error"),
    }
}
