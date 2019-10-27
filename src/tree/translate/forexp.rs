use crate::ast::*;
use crate::tree::*;

pub fn trans_stm<'a>(
    Exp { node, .. }: &Exp,
    mut levels: Vec<Level>,
    mut value_env: ValueEnviroment,
    mut breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::For {
            var,
            lo,
            hi,
            body,
            escape,
        } => {
            let top_level = levels.last_mut().expect("at least a top level");
            let access = top_level.alloc_local(*escape);
            value_env.insert(
                var.clone(),
                EnvEntry::Var {
                    access: access.clone(),
                    depth: top_level.nesting_depth,
                },
            );
            let var_exp = super::varexp::simplevar(access, top_level.nesting_depth, top_level);
            let (lo_exp, lo_frags) = super::trans_exp(
                lo,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                frags,
            )?;
            let (hi_exp, hi_frags) = super::trans_exp(
                hi,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                lo_frags,
            )?;
            let (start_label, continue_label, done_label) = (newlabel(), newlabel(), newlabel());
            breaks_stack.push(Some(done_label));
            let (body_stm, body_frags) =
                super::trans_stm(body, levels, value_env, breaks_stack, hi_frags)?;
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
                body_frags,
            ))
        }
        _ => panic!("not a function call"),
    }
}
