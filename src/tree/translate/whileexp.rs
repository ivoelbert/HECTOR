use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    Exp { node, pos }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::While{test, body} => {
            let (test_cond, test_frags) = super::trans_cond(test, value_env, breaks_stack.clone(), prev_frags)?;
            let (body_stm, body_frags) = super::trans_stm(exp: &Exp, value_env: &ValueEnviroment, breaks_stack: Vec<Option<Label>>, prev_frags: Vec<Fragment>)
        },
        _ => panic!()
    }
}