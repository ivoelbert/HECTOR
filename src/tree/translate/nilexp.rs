use crate::ast::*;
use crate::tree::*;

pub fn trans_exp<'a>(
    Exp {node, ..}: &Exp,
    _levels: Vec<Level>,
    _value_env: ValueEnviroment,
    _breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    // TODO: match for nil
    Ok((CONST(0), frags))
}