use crate::ast::*;
use crate::tree::*;

fn generate_static_link(delta_depth: i32) -> Tree::Exp {
    if delta_depth < 0 {
        panic!("no se matematica")
    }
    if delta_depth == 0 {
        MEM(Box::new(
            plus!(
                TEMP(named_temp(FRAME_POINTER)),
                CONST(STATIC_LINK_OFFSET)
            )
        ))
    } else {
        MEM(Box::new(
            plus!(
                generate_static_link(delta_depth -1),
                CONST(STATIC_LINK_OFFSET)
            )
        ))
    }
}

pub fn trans_exp(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Call { func, args } => {
            let entry = value_env
                .get(func)
                .expect("typecheck should make sure this is found");
            match entry {
                EnvEntry::Func {label, external, depth: callee_depth} => {
                    let caller_depth = level.nesting_depth;
                    let static_link_exp = if *callee_depth > caller_depth {
                        TEMP(named_temp(FRAME_POINTER))
                    } else {
                        generate_static_link(caller_depth - callee_depth)
                    };
                    let (mut arg_exps, args_level, frags) = super::translate_many_exp(args, level, value_env, breaks_stack, frags)?;
                    if *external {
                        Ok((level::external_call(label.clone(), arg_exps), args_level, frags))
                    } else {
                        arg_exps.insert(0, static_link_exp);
                        Ok((CALL(Box::new(NAME(label.clone())), arg_exps), args_level, frags))
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
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    let (exp, exp_level, frags) = trans_exp(exp, level, value_env, breaks_stack, frags)?;
    Ok((EXP(Box::new(exp)), exp_level, frags))
}
