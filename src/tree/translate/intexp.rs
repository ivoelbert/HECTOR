use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    _value_env: &ValueEnviroment,
    _breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::AST, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Int(i) => Ok((CONST(*i), level, frags)),
        _ => panic!()
    }
}