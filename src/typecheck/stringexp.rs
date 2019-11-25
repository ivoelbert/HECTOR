use super::*;

pub fn typecheck(AST{node, pos, ..}: AST, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    match &node {
        Exp::String(..) => Ok(AST {node, pos, typ: Arc::new(TigerType::TString)}),
        _ => panic!("delegation error")
    }
}
