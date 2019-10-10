use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp { node, .. }: &Exp,
    _value_env: &ValueEnviroment,
    _breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Int(i) => Ok((CONST(*i), prev_frags)),
        _ => panic!()
    }
}