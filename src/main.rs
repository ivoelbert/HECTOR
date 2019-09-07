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

fn main() {
    let exp: Exp = WithPos {
        node: OpExp {
            left: Box::new(WithPos {
                node: IntExp(2),
                pos: Pos::new(1, 0),
            }),
            oper: Oper::PlusOp,
            right: Box::new(WithPos {
                node: IntExp(2),
                pos: Pos::new(2, 0),
            }),
        },
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = type_exp(&exp, &type_env, &value_env);
    println!("Expresion {:?}", res);
}
