use super::*;

/// Rebuild an `Exp::Unit` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(AST{node, pos, ..}: AST, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    match &node {
        Exp::Unit => Ok(AST {node, pos, typ: Arc::new(TigerType::TUnit)}),
        _ => panic!("delegation error")
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn unitexp() {
        let ast = make_ast(Exp::Unit);
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }
}