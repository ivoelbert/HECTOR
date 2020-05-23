use super::*;

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    _value_env: &ValueEnviroment,
    _breaks_stack: &[Option<Label>],
    mut frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::String(s) => {
            let l = unique_named_label(s);
            frags.push(Fragment::ConstString(l.clone(), s.clone()));
            Ok((NAME(l), level, frags))
        },
        _ => panic!()
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
    fn stringexp_ok() {
        let exp = AST {
            node: Exp::String(String::from("lorem ipsum")),
            pos: Pos {
                line: 0,
                column: 0,
            },
            typ: Arc::new(TigerType::TString)
        };
        let level = Level::outermost();
        let value_env = initial_value_env();
        let res = translate::stringexp::trans_exp(&exp, level, &value_env, &[], vec![]);
        match res {
            Ok((NAME(_), _level, fragments)) => {
                assert!(!fragments.is_empty());
            },
            Ok(..) => panic!("wrong result"),
            Err(..) => panic!("should translate"),
        }
    }
}