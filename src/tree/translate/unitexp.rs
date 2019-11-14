use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp {node, ..}: &Exp,
    level: Level,
    _value_env: &ValueEnviroment,
    _breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    // Translates as a noop.
    match node {
        _Exp::Unit => Ok((CONST(0), level, frags)),
        _ => panic!("delegation error")
    }
}