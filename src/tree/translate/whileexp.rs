use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    AST {node, ..}: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::While{test, body} => {
            let (test_label, body_label, done_label) = (newlabel(), newlabel(), newlabel());
            let (test_exp, test_level, test_frags) = super::trans_exp(test, level, value_env, breaks_stack, frags)?;
            let mut new_breaks_stack = breaks_stack.clone();
            new_breaks_stack.push(Some(done_label));
            let (body_stm, body_level, body_frags) = super::trans_stm(body, test_level, value_env, &new_breaks_stack, test_frags)?;
            Ok((seq(vec![
                LABEL(test_label),
                CJUMP(GE, test_exp, CONST(1), body_label, done_label),
                LABEL(body_label),
                body_stm,
                JUMP(NAME(test_label), vec![test_label]),
                LABEL(done_label)
            ]), body_level, body_frags))
        },
        _ => panic!()
    }
}