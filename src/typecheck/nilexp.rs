use super::*;

pub fn typecheck(AST{node, pos, ..}: AST, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    match &node {
        Exp::Nil => Ok(AST {node, pos, typ: Arc::new(TigerType::TNil)}),
        _ => panic!("delegation error")
    }
}