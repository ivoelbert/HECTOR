use crate::ast::*;
use crate::tree::*;
use Tree::Exp::*;

pub fn trans_var(
    var: &Var,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    //TODO
    match var {
        Var::Simple(name) => {
            Ok((CONST(0), prev_frags))
        },
        Var::Subscript(array, index) => {
            Ok((CONST(0), prev_frags))
        },
        Var::Field(record, field) => {
            Ok((CONST(0), prev_frags))
        },
        _ => panic!()
    }
}