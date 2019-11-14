use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp {node, ..}: &Exp,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    Ok((CONST(0), level, frags)) // TODO
    // hay que llamar una external que pida memeoria dinamica
}