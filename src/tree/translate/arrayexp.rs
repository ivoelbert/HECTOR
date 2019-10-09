use crate::ast::*;
use crate::tree::*;
use crate::tree::translate::*;
use ExpInterm::*;

pub fn translate(Exp{node, ..}: Exp) -> Result<ExpInterm, TransError> {
    match node {
        _Exp::Array{size, init, ..} => {
            let i = un_ex(trans_exp(*init.clone())?);
            let s = un_ex(trans_exp(*size.clone())?);
            Ok(Ex (Frame::external_call(String::from("allocArray"), vec![s, i])))
        },
        _ => panic!()
    }
}