use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    AST { node, .. }: &AST,
    mut level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::For {
            var,
            lo,
            hi,
            body,
            escape,
        } => {
            let mut new_value_env = value_env.clone();
            let mut new_breaks_stack = breaks_stack.to_vec();
            let access = level.alloc_local(*escape, Some(var.clone()));
            new_value_env.insert(
                var.clone(),
                EnvEntry::Var {
                    access: access.clone(),
                    depth: level.nesting_depth,
                },
            );
            let var_exp = super::varexp::simplevar(access, &level, level.nesting_depth);
            let (lo_exp, lo_level, lo_frags) = super::trans_exp(
                lo,
                level,
                &new_value_env,
                breaks_stack,
                frags,
            )?;
            let (hi_exp, hi_level, hi_frags) = super::trans_exp(
                hi,
                lo_level,
                &new_value_env,
                breaks_stack,
                lo_frags,
            )?;
            let (start_label, continue_label, done_label) = (unique_named_label("-for-start"), unique_named_label("-for-continue"), unique_named_label("-for-done"));
            new_breaks_stack.push(Some(done_label.clone()));
            let (body_stm, body_level, body_frags) =
                super::trans_stm(body, hi_level, &new_value_env, &new_breaks_stack, hi_frags)?;
            // TODO: This is super inefficient. It repeats computacion of lo and hi.
            // Also, maybe it's incorrect
            // We should move them to temps
            Ok((
                Tree::seq(vec![
                    Move!(var_exp.clone(), lo_exp),
                    CJUMP(LE,
                        Box::new(var_exp.clone()),
                        Box::new(hi_exp.clone()),
                        start_label.clone(),
                        done_label.clone(),
                    ),
                    LABEL(start_label.clone()),
                    body_stm,
                    CJUMP(LT, Box::new(var_exp.clone()), Box::new(hi_exp), continue_label.clone(), done_label.clone()),
                    LABEL(continue_label),
                    Move!(var_exp.clone(), plus!(var_exp, CONST(1))),
                    JUMP(NAME(start_label.clone()), vec![start_label]),
                    LABEL(done_label),
                ]),
                body_level,
                body_frags,
            ))
        }
        _ => panic!("not a function call"),
    }
}
