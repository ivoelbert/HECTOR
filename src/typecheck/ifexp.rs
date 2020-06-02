use super::*;

/// Rebuild an `Exp::If` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    match node {
        Exp::If{test, then_, else_} => {
            let test_ast = type_exp(*test, type_env, value_env)?;
            if !es_int(&tipo_real(Arc::clone(&test_ast.typ), type_env)) {
                return Err(TypeError::NonIntegerCondition(pos));
            }
            let then_ast = type_exp(*then_, type_env, value_env)?;
            match else_ {
                Some(else_exp) => {
                    let else_ast = type_exp(*else_exp, type_env, value_env)?;
                    if else_ast.typ != then_ast.typ {
                        console_log!("if mismatch");
                        return Err(TypeError::ThenElseTypeMismatch(pos))
                    }
                    let typ = Arc::clone(&then_ast.typ);
                    Ok(AST {
                        node: Exp::If {
                            test: Box::new(test_ast),
                            then_: Box::new(then_ast),
                            else_: Some(Box::new(else_ast)),
                        },
                        pos,
                        typ
                    })
                }
                None => if *then_ast.typ == TigerType::TUnit {
                    let typ = Arc::clone(&then_ast.typ);
                    Ok(AST {
                        node: Exp::If {
                            test: Box::new(test_ast),
                            then_: Box::new(then_ast),
                            else_: None
                        },
                        pos,
                        typ
                    })
                } else {
                    Err(TypeError::NonUnitBody(pos))
                }
            }
        }
        _ => panic!("Delegation error on ifexp::tipar")
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn ifexp_ok() {
        let ast = make_ast(Exp::If {
            test: boxed_ast(Exp::Int(0)),
            then_: boxed_ast(Exp::Int(1)),
            else_: Some(boxed_ast(Exp::Int(2)))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn ifexp_non_integer_test() {
        let ast = make_ast(Exp::If {
            test: boxed_ast(Exp::String(String::from("perro"))),
            then_: boxed_ast(Exp::Int(1)),
            else_: Some(boxed_ast(Exp::Int(2)))
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

    #[test]
    #[wasm_bindgen_test]
    fn ifexp_type_mismatch() {
        let ast = make_ast(Exp::If {
            test: boxed_ast(Exp::Int(0)),
            then_: boxed_ast(Exp::Int(1)),
            else_: Some(boxed_ast(Exp::String(String::from("perro")))),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::ThenElseTypeMismatch(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn ifexp_non_unit_body() {
        let ast = make_ast(Exp::If {
            test: boxed_ast(Exp::Int(0)),
            then_: boxed_ast(Exp::Int(1)),
            else_: None
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NonUnitBody(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}