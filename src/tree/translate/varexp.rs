use crate::ast::*;
use crate::tree::*;
use ExpInterm::*;
use Tree::Exp::*;
use Tree::Stm::*;


pub fn trans_var(var: Var) -> Result<ExpInterm, TransError> {
    //TODO
    match var {
        Var::Simple(name) => {
            Ok(Ex(CONST(0)))
        },
        Var::Subscript(array, index) => {
            Ok(Ex(CONST(0)))
        },
        Var::Field(record, field) => {
            Ok(Ex(CONST(0)))
        },
        _ => panic!()
    }
}

pub fn translate(Exp{node, ..}: Exp) -> Result<ExpInterm, TransError> {
    match node {
        _Exp::Var(v) => {
            trans_var(v)
        },
        _ => panic!()
    }
}