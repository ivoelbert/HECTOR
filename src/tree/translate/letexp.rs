use crate::ast::*;
use crate::tree::*;

pub fn vardec(
    (
        _VarDec {
            name, escape, init, ..
        },
        pos,
    ): &(_VarDec, Pos),
    top_level: &mut Level,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    Ok((EXP(Box::new(CONST(0))), frags)) // TODO
}

pub fn fundecs(
    funcs: &[(_FunctionDec, Pos)],
    top_level: &mut Level,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    Ok((EXP(Box::new(CONST(0))), frags)) // TODO
}

pub fn trans_exp<'a>(
    Exp { node, .. }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Let { decs, body } => Ok((CONST(0), frags)),
        _ => panic!("delegation error"),
    }
}
