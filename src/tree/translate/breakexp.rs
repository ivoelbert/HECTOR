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

#[test]
fn no_labels_error() {
    let exp = Exp {node: _Exp::Break, pos: Pos {line: 0, column: 0}};
    let value_env = initial_value_env();
    let res = trans_stm(&exp, &value_env, vec![], vec![]);
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
fn none_label_error() {
    let exp = Exp {node: _Exp::Break, pos: Pos {line: 0, column: 0}};
    let value_env = initial_value_env();
    let res = trans_stm(&exp, &value_env, vec![None], vec![]);
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
fn ok() {
    let exp = Exp {node: _Exp::Break, pos: Pos {line: 0, column: 0}};
    let value_env = initial_value_env();
    let res = trans_stm(&exp, &value_env, vec![Some(Label::from("done"))], vec![]);
    match res {
        Ok((tree, fragments)) => {
            assert_eq!(tree, JUMP(NAME(Label::from("done")), vec![Label::from("done")]));
            assert!(fragments.is_empty());
        },
        Err(..) => panic!("should translate"),
    }
}