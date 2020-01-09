use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    _value_env: &ValueEnviroment,
    _breaks_stack: &Vec<Option<Label>>,
    mut frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::String(s) => {
            let l = newlabel();
            // Not sure if this is OK or I need one more fragment for the length
            frags.push(Fragment::ConstString(l.clone(), s.clone()));
            Ok((NAME(l), level, frags))
        },
        _ => panic!()
    }
}