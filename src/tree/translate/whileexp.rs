use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    Exp { node, .. }: &Exp,
    value_env: &ValueEnviroment,
    mut breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::While{test, body} => {
            let (test_label, body_label, done_label) = (newlabel(), newlabel(), newlabel());
            let (test_cond, test_frags) = super::trans_cond(test, value_env, breaks_stack.clone(), prev_frags)?;
            breaks_stack.push(Some(done_label.clone()));
            let (body_stm, body_frags) = super::trans_stm(body, value_env, breaks_stack, test_frags)?;
            Ok((seq(vec![
                LABEL(test_label.clone()),
                test_cond(body_label.clone(), done_label.clone()),
                LABEL(body_label),
                body_stm,
                JUMP(NAME(test_label.clone()), vec![test_label]),
                LABEL(done_label)
            ]), body_frags))
        },
        _ => panic!()
    }
}