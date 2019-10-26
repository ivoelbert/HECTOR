use crate::ast::*;
use crate::tree::*;

pub fn trans_exp<'a>(
    Exp { node, .. }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    _breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Call { func, args } => {
            let entry = value_env
                .get(func)
                .expect("typecheck should make sure this is found");
            match entry {
                EnvEntry::Func {
                    label,
                    level,
                    external,
                } => {
                    Ok((CONST(0), frags)) // TODO
                }
                EnvEntry::Var { .. } => panic!("typecheck should make sure this is a function"),
            }
        }
        _ => panic!("not a function call"),
    }
}

pub fn trans_stm<'a>(
    Exp { node, pos }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    mut breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    Ok((EXP(Box::new(CONST(0))), frags)) // TODO
}
