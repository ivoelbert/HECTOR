use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp { node, pos }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    Ok((CONST(0), prev_frags))
}