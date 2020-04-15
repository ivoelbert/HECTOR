use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    match node {
        Exp::While {test, body} => {
            let test_ast = type_exp(*test, type_env, value_env)?;
            if !es_int(&tipo_real(test_ast.typ.clone(), type_env)) {
                return Err(TypeError::NonIntegerCondition(pos));
            }
            let body_ast = type_exp(*body, type_env, value_env)?;
            match *body_ast.typ {
                TigerType::TUnit => Ok(AST {
                    node: Exp::While{
                        test: Box::new(test_ast),
                        body: Box::new(body_ast)
                    },
                    typ: Arc::new(TigerType::TUnit),
                    pos
                }),
                _ => Err(TypeError::NonUnitBody(pos))
            }
        }
        _ => panic!("le llego cualquier cosa a whileexp::tipar")
    }
}


#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn whileexp_ok() {
        let ast = make_ast(Exp::While {
            test: boxed_ast(Exp::Int(0)),
            body: boxed_ast(Exp::Unit),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn whileexp_non_integer_condition() {
        let ast = make_ast(Exp::While {
            test: boxed_ast(Exp::Unit),
            body: boxed_ast(Exp::Unit),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NonIntegerCondition(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}