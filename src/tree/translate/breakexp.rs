use super::*;

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

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    use std::sync::Arc;

    #[test]
    #[wasm_bindgen_test]
    fn break_no_labels_error() {
        let exp = AST {
            node: Exp::Break,
            pos: Pos { line: 0, column: 0 },
            typ: Arc::new(TigerType::TUnit)
        };
        let level = Level::outermost();
        let res = translate::breakexp::trans_stm(&exp, level, &initial_value_env(), &vec![], vec![]);
        match res {
            Err(TransError::BreakError(_)) => (),
            Err(..) => panic!("wrong error"),
            Ok(..) => panic!("shouldn't translate"),
        }
    }

    #[test]
    fn break_none_label_error() {
        let exp = AST {
            node: Exp::Break,
            pos: Pos { line: 0, column: 0 },
            typ: Arc::new(TigerType::TUnit)
        };
        let level = Level::outermost();
        let res = translate::breakexp::trans_stm(&exp, level, &initial_value_env(), &vec![], vec![]);
        match res {
            Err(TransError::BreakError(_)) => (),
            Err(..) => panic!("wrong error"),
            Ok(..) => panic!("shouldn't translate"),
        }
    }

    #[test]
    fn break_ok() {
        let exp = AST {
            node: Exp::Break,
            pos: Pos { line: 0, column: 0 },
            typ: Arc::new(TigerType::TUnit)
        };
        let level = Level::outermost();
        let res = translate::breakexp::trans_stm(&exp, level, &initial_value_env(), &vec![Some(unique_named_label("-break"))], vec![]);
        match res {
            Ok((JUMP(NAME(_), _), _, fragments)) => {
                assert!(fragments.is_empty());
            }
            Ok(..) => panic!("wrong translation"),
            Err(..) => panic!("should translate"),
        }
    }
}