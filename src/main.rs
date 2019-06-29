mod ast;

use ast::tigerabs::*;
use ast::tigerabs::_Exp::*;
use ast::position::{Pos, WithPos};

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

    println!("Expresion {:?}", exp);
}
