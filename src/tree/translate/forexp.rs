use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    Exp { node, .. }: &Exp,
    mut level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        _Exp::For {
            var,
            lo,
            hi,
            body,
            escape,
        } => {
            let mut new_value_env = value_env.clone();
            let mut new_breaks_stack = breaks_stack.clone();
            let access = level.alloc_local(*escape);
            new_value_env.insert(
                var.clone(),
                EnvEntry::Var {
                    access: access.clone(),
                    depth: level.nesting_depth,
                },
            );
            let var_exp = super::varexp::simplevar(access, level.nesting_depth, &level);
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
            let (start_label, continue_label, done_label) = (newlabel(), newlabel(), newlabel());
            new_breaks_stack.push(Some(done_label));
            let (body_stm, body_level, body_frags) =
                super::trans_stm(body, hi_level, value_env, &new_breaks_stack, hi_frags)?;
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        LE,
                        lo_exp.clone(),
                        hi_exp.clone(),
                        start_label,
                        done_label,
                    ),
                    LABEL(start_label),
                    body_stm,
                    CJUMP(LT, lo_exp, hi_exp, start_label, done_label),
                    LABEL(continue_label),
                    Move!(var_exp.clone(), plus!(var_exp, CONST(1))),
                    JUMP(NAME(start_label), vec![start_label]),
                    LABEL(done_label),
                ]),
                body_level,
                body_frags,
            ))
        }
        _ => panic!("not a function call"),
    }
}
