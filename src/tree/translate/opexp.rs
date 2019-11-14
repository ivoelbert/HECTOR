use crate::ast::*;
use crate::tree::*;
use crate::typecheck::{type_exp, TigerType};

pub fn trans_exp(
    Exp {node, ..}: &Exp,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        _Exp::Op{left, right, oper} => {
            // match oper {
            // }
        }
        _ => panic!("delegation error")
    };
    Ok((CONST(0), level, frags)) // TODO
    // mapeo
    // hay que volver a tipar para diferenciar
}