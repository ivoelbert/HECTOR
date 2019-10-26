use crate::ast::*;
use crate::tree::*;

pub fn trans_exp<'a>(
    Exp {node, ..}: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    Ok((CONST(0), frags))
}