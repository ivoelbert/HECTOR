use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp {node, ..}: &Exp,
    level: Level,
    _value_env: &ValueEnviroment,
    _breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        _Exp::Int(i) => Ok((CONST(*i), level, frags)),
        _ => panic!()
    }
}