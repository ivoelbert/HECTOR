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
            let (test_label, body_label, done_label) = (unique_named_label("-test"), unique_named_label("-body"), unique_named_label("-while-done"));
            let (test_exp, test_level, test_frags) = super::trans_exp(test, level, value_env, breaks_stack, frags)?;
            let mut new_breaks_stack = breaks_stack.clone();
            new_breaks_stack.push(Some(done_label.clone()));
            let (body_stm, body_level, body_frags) = super::trans_stm(body, test_level, value_env, &new_breaks_stack, test_frags)?;
            Ok((seq(vec![
                LABEL(test_label.clone()),
                CJUMP(GE, Box::new(test_exp), Box::new(CONST(1)), body_label.clone(), done_label.clone()),
                LABEL(body_label.clone()),
                body_stm,
                JUMP(NAME(test_label.clone()), vec![test_label.clone()]),
                LABEL(done_label.clone())
            ]), body_level, body_frags))
        },
        _ => panic!()
    }
}