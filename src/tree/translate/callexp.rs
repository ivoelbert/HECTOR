use crate::ast::*;
use crate::tree::*;
use crate::utils::log;

pub fn trans_exp(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Call { func, args } => {
            let entry = value_env
                .get(func)
                .expect("typecheck should make sure this is found");
            match entry {
                EnvEntry::Func {label, external} => {
                    let (arg_exps, args_level, frags) = super::translate_many_exp(args, level, value_env, breaks_stack, frags)?;
                    if *external {
                        Ok((level::external_call(label.clone(), label.clone(), arg_exps), args_level, frags))
                    } else {
                        Ok((CALL(String::from(func), Box::new(NAME(label.clone())), arg_exps), args_level, frags))
                    }
                }
                EnvEntry::Var { .. } => {
                    panic!("typecheck should make sure this is a function")
                },
            }
        }
        _ => panic!("not a function call"),
    }
}

pub fn trans_stm(
    exp: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    let (exp, exp_level, frags) = trans_exp(exp, level, value_env, breaks_stack, frags)?;
    Ok((EXP(Box::new(exp)), exp_level, frags))
}
