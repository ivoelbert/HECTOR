use crate::ast::*;
use crate::tree::*;
use super::varexp::simplevar;

pub fn vardec(
    (_VarDec {name, escape, init, ..}, _,): (&_VarDec, &Pos),
    level: Level,
    mut value_env: ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, ValueEnviroment, Vec<Fragment>), TransError> {
    let (init_exp, mut init_level, init_frags) = super::trans_exp(init, level, &value_env, breaks_stack, frags)?;
    // We don't want the variable to be in the env when we translate the initialization
    let access = init_level.alloc_local(*escape, Some(name.clone()));
    value_env.insert(name.clone(), EnvEntry::Var{
        access: access.clone(),
        depth: init_level.nesting_depth
    });

    Ok((Move!(simplevar(access.clone(), &init_level), init_exp), init_level, value_env, init_frags))
}

pub fn fundecs(
    funcs: &[(_FunctionDec, Pos)],
    depth: i32,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(ValueEnviroment, Vec<Fragment>), TransError> {
    // Add a new entry to the breaks stack (so that a break fails)
    // Add all functions to a new env, declaring labels for each one.
    // Translate each funtion
    // Return all the new fragments
    let mut new_breaks_stack = breaks_stack.clone();
    new_breaks_stack.push(None);
    let mut new_value_env = value_env.clone();
    funcs.iter().for_each(|(_FunctionDec {name, ..}, _)| {
        new_value_env.insert(name.clone(), EnvEntry::Func{
            label: unique_named_label(name),
            external: false,
        });
    });
    let new_frags = funcs
        .iter()
        .try_fold(frags, |frags, (_FunctionDec{name, params, body, result}, _)|
        -> Result<Vec<Fragment>, TransError> {
            // Get the funciton label
            // Create a new level
            // Allocate the parameters in the level
            // Translate the body in an env with the parameters
            // Close the level in a fragment.
            let label = if let EnvEntry::Func{label, ..} = new_value_env.get(name).unwrap() {
                label
            } else {panic!()};
            let mut level = Level::new(depth + 1, name.clone(), label.clone());
            let mut dec_value_env = new_value_env.clone();
            params
                .iter()
                .for_each(|Field{name, escape, ..}| {
                    let access = level.alloc_arg(name.clone(), *escape);
                    dec_value_env.insert(name.clone(), EnvEntry::Var{
                        access,
                        depth: depth + 1
                    });
                });
            match result {
                // If the function doesn't have a return value, then don't move a return value
                Some(_) => {
                    let (body_exp, body_level, mut body_frags) = super::trans_exp(body, level, &dec_value_env, &new_breaks_stack, frags)?;
                    let move_exp = Move!(GLOBAL(RETURN_VALUE.to_string()), body_exp);
                    let fragment = Fragment::new(move_exp, body_level);
                    body_frags.push(fragment);
                    Ok(body_frags)
                }
                None => {
                    let (body_stm, body_level, mut body_frags) = super::trans_stm(body, level, &dec_value_env, &new_breaks_stack, frags)?;
                    let fragment = Fragment::new(body_stm, body_level);
                    body_frags.push(fragment);
                    Ok(body_frags)
                }
            }
        })?;
    Ok((new_value_env, new_frags))
}

pub fn trans_exp(
    AST { node, .. }: &AST,
    mut level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    mut frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Let { decs, body } => {
            // REFACTOR: this should be a fold
            let mut vardec_stms = vec![];
            let mut new_value_env = value_env.clone();
            for dec in decs {
                match dec {
                    Dec::VarDec(vd, pos) => {
                        let (exp, l, ve, f) = vardec((vd, pos), level, new_value_env, breaks_stack, frags)?;
                        vardec_stms.push(exp);
                        new_value_env = ve;
                        level = l;
                        frags = f;
                    },
                    Dec::FunctionDec(fd) => {
                        let (ve, f) = fundecs(fd, level.nesting_depth + 1, &new_value_env, breaks_stack, frags)?;
                        new_value_env = ve;
                        frags = f;
                    },
                    Dec::TypeDec(_) => (),
                }
            }
            let (body_exp, body_level, body_frags) = super::trans_exp(body, level, &new_value_env, breaks_stack, frags)?;
            let let_exp = ESEQ(Box::new(seq(vardec_stms)), Box::new(body_exp));
            Ok((let_exp, body_level, body_frags))
        },
        _ => panic!("delegation error"),
    }
}
