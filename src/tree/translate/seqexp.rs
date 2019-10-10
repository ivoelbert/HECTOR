use crate::ast::*;
use crate::tree::*;


pub fn trans_stm(
    Exp { node, pos }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    Ok((EXP(Box::new(CONST(0))), prev_frags))
}