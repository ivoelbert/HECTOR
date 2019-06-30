mod ast;
mod seman;

#[cfg(test)]
mod test;

use ast::tigerabs::*;
use ast::tigerabs::_Exp::*;
use ast::position::{Pos, WithPos};

use seman::types::{Tipo, R};

fn main() {
    /*let exp: Exp = WithPos {
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

    println!("Expresion {:?}", exp);
    */
    let t1 : seman::types::Tipo = Tipo::TUnit;
    let t2 = Tipo::TInt(R::RO);
    let t3 = Tipo::TInt(R::RW);
    println!("{:?}", t2 == t3)
}
