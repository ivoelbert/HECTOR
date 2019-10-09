use crate::ast::*;
use crate::tree::*;
use ExpInterm::*;
use Tree::Exp::*;

pub fn translate(Exp{node, ..}: Exp) -> Result<ExpInterm, TransError> {
    match node {
        _Exp::Int(i) => Ok(Ex(CONST(i))),
        _ => panic!()
    }
}