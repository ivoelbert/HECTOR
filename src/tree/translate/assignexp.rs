use super::varexp::trans_var;
use super::*;
use Tree::Stm::*;

pub fn trans_stm(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Assign { var, exp } => {
            let (v, var_level, var_frags) = trans_var(var, level, value_env, breaks_stack, frags)?;
            let (e, exp_level, exp_frags) = super::trans_exp(exp, var_level, value_env, breaks_stack, var_frags)?;
            Ok((Move!(v, e), exp_level, exp_frags))
        }
        _ => panic!(),
    }
}
