use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp {node, ..}: &Exp,
    level: Level,
    _value_env: &ValueEnviroment,
    _breaks_stack: &Vec<Option<Label>>,
    mut frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        _Exp::String(s) => {
            let l = newlabel();
            // Not sure if this is OK or I need one more fragment for the length
            frags.push(Fragment::ConstString(l.clone(), s.clone()));
            Ok((NAME(l), level, frags))
        },
        _ => panic!()
    }
}

#[test]
fn ok() {
    let exp = Exp {
        node: _Exp::String(String::from("lorem ipsum")),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let level = Level::outermost();
    let value_env = initial_value_env();
    let res = trans_exp(&exp, level, &value_env, &vec![], vec![]);
    match res {
        Ok((NAME(_), _level, fragments)) => {
            assert!(!fragments.is_empty());
        },
        Ok(..) => panic!("wrong result"),
        Err(..) => panic!("should translate"),
    }
}