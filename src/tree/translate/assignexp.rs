use crate::ast::*;
use super::*;
use ExpInterm::*;
use Tree::Stm::*;
use super::varexp::trans_var;

pub fn translate(Exp{node, ..}: Exp) -> Result<ExpInterm, TransError> {
    match node {
        _Exp::Assign{var, exp} => {
            let v = un_ex(trans_var(var)?);
            let e = un_ex(trans_exp(*exp)?);
            Ok(Nx(MOVE(Box::new(v), Box::new(e))))
        },
        _ => panic!()
    }
}