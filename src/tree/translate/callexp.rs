use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp { node, .. }: &Exp,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        _Exp::Call { func, args } => {
            let entry = value_env
                .get(func)
                .expect("typecheck should make sure this is found");
            match entry {
                EnvEntry::Func {label, external: _} => {
                    let (mut arg_exps, args_level, frags) = super::translate_many_exp(args, level, value_env, breaks_stack, frags)?;
                    let sl = super::varexp::generate_static_link(args_level.nesting_depth);
                    arg_exps.insert(0, sl);
                    Ok((CALL(Box::new(NAME(*label)), arg_exps), args_level, frags))

                    // TODO: external calls
                }
                EnvEntry::Var { .. } => panic!("typecheck should make sure this is a function"),
            }
        }
        _ => panic!("not a function call"),
    }
}

pub fn trans_stm(
    exp: &Exp,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    let (exp, exp_level, frags) = trans_exp(exp, level, value_env, breaks_stack, frags)?;
    Ok((EXP(Box::new(exp)), exp_level, frags))
}
