use crate::ast::*;
use crate::tree::*;

pub fn trans_stm<'a>(
    Exp { node, pos }: &Exp,
    _levels: Vec<Level>,
    _value_env: ValueEnviroment,
    mut breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::Break => {
            let loop_end_label = match breaks_stack.pop() {
                Some(Some(l)) => l,
                _ => return Err(TransError::BreakError(*pos)),
            };
            Ok((
                JUMP(NAME(loop_end_label), vec![loop_end_label]),
                frags,
            ))
        }
        _ => panic!(),
    }
}

#[test]
fn no_labels_error() {
    let exp = Exp {
        node: _Exp::Break,
        pos: Pos { line: 0, column: 0 },
    };
    let level = Level::outermost();
    let res = trans_stm(
        &exp,
        vec![level.clone()],
        initial_value_env(level),
        vec![],
        vec![],
    );
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
fn none_label_error() {
    let exp = Exp {
        node: _Exp::Break,
        pos: Pos { line: 0, column: 0 },
    };
    let level = Level::outermost();
    let res = trans_stm(
        &exp,
        vec![level.clone()],
        initial_value_env(level),
        vec![],
        vec![],
    );
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
fn ok() {
    let exp = Exp {
        node: _Exp::Break,
        pos: Pos { line: 0, column: 0 },
    };
    let level = Level::outermost();
    let res = trans_stm(
        &exp,
        vec![level.clone()],
        initial_value_env(level),
        vec![],
        vec![],
    );
    match res {
        Ok((JUMP(NAME(_), _), fragments)) => {
            assert!(fragments.is_empty());
        }
        Ok(..) => panic!("wrong translation"),
        Err(..) => panic!("should translate"),
    }
}
