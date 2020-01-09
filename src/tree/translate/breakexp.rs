use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    AST { node, pos, .. }: &AST,
    level: Level,
    _value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Break => {
            let loop_end_label = match breaks_stack.last() {
                Some(Some(l)) => l,
                _ => return Err(TransError::BreakError(*pos)),
            };
            Ok((
                JUMP(NAME(loop_end_label.clone()), vec![loop_end_label.clone()]),
                level,
                frags,
            ))
        }
        _ => panic!(),
    }
}