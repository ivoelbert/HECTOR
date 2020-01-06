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

#[test]
fn no_labels_error() {
    let exp = AST {
        node: Exp::Break,
        pos: Pos { line: 0, column: 0 },
        typ: Arc::new(TigerType::TUnit)
    };
    let level = Level::outermost();
    let res = trans_stm(&exp, level, &initial_value_env(), &vec![], vec![]);
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
fn none_label_error() {
    let exp = AST {
        node: Exp::Break,
        pos: Pos { line: 0, column: 0 },
        typ: Arc::new(TigerType::TUnit)
    };
    let level = Level::outermost();
    let res = trans_stm(&exp, level, &initial_value_env(), &vec![], vec![]);
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
fn ok() {
    let exp = AST {
        node: Exp::Break,
        pos: Pos { line: 0, column: 0 },
        typ: Arc::new(TigerType::TUnit)
    };
    let level = Level::outermost();
    let res = trans_stm(&exp, level, &initial_value_env(), &vec![Some(newlabel())], vec![]);
    match res {
        Ok((JUMP(NAME(_), _), _, fragments)) => {
            assert!(fragments.is_empty());
        }
        Ok(..) => panic!("wrong translation"),
        Err(..) => panic!("should translate"),
    }
}
