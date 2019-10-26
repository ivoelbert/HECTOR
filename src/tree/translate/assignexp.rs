use super::varexp::trans_var;
use super::*;
use Tree::Stm::*;

pub fn trans_stm<'a>(
    Exp { node, .. }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::Assign { var, exp } => {
            let (v, var_frags) = trans_var(
                var,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                frags,
            )?;
            let (e, exp_frags) = super::trans_exp(exp, levels, value_env, breaks_stack, var_frags)?;
            Ok((Move!(v, e), exp_frags))
        }
        _ => panic!(),
    }
}
