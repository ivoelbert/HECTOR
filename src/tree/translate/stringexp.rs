use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    Exp { node, .. }: &Exp,
    _value_env: &ValueEnviroment,
    _breaks_stack: Vec<Option<Label>>,
    mut prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::String(s) => {
            let l = newlabel();
            // Not sure if this is OK or I need one more fragment for the length
            prev_frags.push(Fragment::ConstString(l.clone(), s.clone()));
            Ok((NAME(l), prev_frags))
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
    let value_env = initial_value_env();
    let res = trans_exp(&exp, &value_env, vec![Some(Label::from("done"))], vec![]);
    match res {
        Ok((NAME(_), fragments)) => {
            assert!(!fragments.is_empty());
        },
        Ok(..) => panic!("wrong result"),
        Err(..) => panic!("should translate"),
    }
}