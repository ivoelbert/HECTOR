use super::*;

pub fn typecheck(AST{node, pos, ..}: AST, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    match &node {
        Exp::Break => Ok(AST {node, pos, typ: Arc::new(TigerType::TUnit)}),
        _ => panic!("delegation error")
    }
}