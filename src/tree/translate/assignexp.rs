use crate::ast::*;
use super::*;
use Tree::Stm::*;
use super::varexp::trans_var;

pub fn trans_stm(
    Exp { node, .. }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::Assign{var, exp} => {
            let (v, var_frags) = trans_var(var, value_env, breaks_stack.clone(), prev_frags)?;
            let (e, exp_frags) = trans_exp(exp, value_env, breaks_stack, var_frags)?;
            Ok((MOVE(Box::new(v), Box::new(e)), exp_frags))
        },
        _ => panic!()
    }
}