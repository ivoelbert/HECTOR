use super::*;

/// Rebuild an `Exp::Nil` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(AST{node, pos, ..}: AST, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    match &node {
        Exp::Nil => Ok(AST {node, pos, typ: Arc::new(TigerType::TNil)}),
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
    fn nilexp() {
        let ast = make_ast(Exp::Nil);
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) => match *typ {
                TigerType::TNil => (),
                _ => panic!("wrong type: {:?}", typ),
            },
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }
}