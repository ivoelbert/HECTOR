use crate::ast::*;
use crate::tree::*;

pub fn trans_stm<'a>(
    Exp {node, ..}: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    mut breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::While{test, body} => {
            let (test_label, body_label, done_label) = (newlabel(), newlabel(), newlabel());
            let (test_exp, test_frags) = super::trans_exp(test, levels.clone(), value_env.clone(), breaks_stack.clone(), frags)?;
            breaks_stack.push(Some(done_label));
            let (body_stm, body_frags) = super::trans_stm(body, levels, value_env, breaks_stack, test_frags)?;
            Ok((seq(vec![
                LABEL(test_label),
                CJUMP(GE, test_exp, CONST(1), body_label, done_label),
                LABEL(body_label),
                body_stm,
                JUMP(NAME(test_label), vec![test_label]),
                LABEL(done_label)
            ]), body_frags))
        },
        _ => panic!()
    }
}