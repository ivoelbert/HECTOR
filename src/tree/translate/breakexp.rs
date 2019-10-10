use crate::ast::*;
use crate::tree::*;

pub fn trans_stm(
    Exp { node, pos }: &Exp,
    _value_env: &ValueEnviroment,
    mut breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::Break => {
            let loop_end_label = match breaks_stack.pop() {
                Some(Some(l)) => l,
                _ => return Err(TransError::BreakError(*pos))
            };
            Ok((JUMP(NAME(loop_end_label.clone()), vec![loop_end_label]), prev_frags))
        }
        _ => panic!(),
    }
}