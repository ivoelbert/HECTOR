use crate::ast::*;
use crate::tree::*;
use crate::typecheck::{type_exp, TigerType};

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::AST, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Op{left, right, oper} => {
            // match oper {
            // }
        }
        _ => panic!("delegation error")
    };
    Ok((CONST(0), level, frags)) // TODO
    // mapeo
    // hay que volver a tipar para diferenciar
}