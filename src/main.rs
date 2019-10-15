
#[allow(dead_code)]
mod ast;
#[allow(dead_code)]
mod seman;
#[cfg(test)]
mod test;

use ast::tigerabs::*;
use ast::tigerabs::_Exp::*;
use ast::position::{Pos, WithPos};

use seman::tigerseman::*;

//#[macro_use]
//extern crate lalrpop_util;

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
    let res = tipar_exp(exp, type_env, value_env);
    println!("Expresion {:?}", res);
}
