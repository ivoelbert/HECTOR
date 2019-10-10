use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    _exp: &Exp,
    _value_env: &ValueEnviroment,
    _breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    Ok((CONST(0), prev_frags))
}