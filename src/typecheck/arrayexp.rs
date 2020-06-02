use crate::typecheck::*;

/// Rebuild an `Exp::Array` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(
    AST {node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    // Get array type in value_env. If it's not an array, fail
    // Type the size. If it's not int, fail
    // Type the init. If it's not the same as the array, fail
    // Return TArray of the type found in hte env
    use TigerType::*;
    match node {
        Exp::Array {typ: array_of_symbol, size: size_exp, init: init_exp, } => {
            let array_type = match type_env.get(&array_of_symbol) {
                Some(tiger_type) => &**tiger_type,
                None => {
                    console_log!("array undeclared");
                    return Err(TypeError::UndeclaredType(pos))
                },
            };
            match array_type {
                TArray(array_of_type, type_id) => {
                    let size_ast = type_exp(*size_exp, type_env, value_env)?;
                    match *size_ast.typ {
                        TInt(_) => {
                            let init_ast = type_exp(*init_exp, type_env, value_env)?;
                            if **array_of_type == *init_ast.typ {
                                Ok(AST{
                                    node: Exp::Array {
                                        size: Box::new(size_ast),
                                        init: Box::new(init_ast),
                                        typ: array_of_symbol
                                    },
                                    pos,
                                    typ: Arc::new(TArray(Arc::<TigerType>::clone(&array_of_type), *type_id))})
                            } else {
                                console_log!("array mismatch");
                                Err(TypeError::TypeMismatch(pos))
                            }
                        }
                        TigerType::TUnit
                        | TigerType::TNil
                        | TigerType::TString
                        | TigerType::TArray(..)
                        | TigerType::TRecord(..)
                        | TigerType::Internal(..)
                        | TigerType::Untyped => Err(TypeError::NonIntegerSize(pos)),
                    }
                },
                _ => Err(TypeError::NotArrayType(pos)),
            }
        }
        _ => panic!("le llego algo nada que ver a arrayexp::tipar"),
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn arrayexp_ok() {
        let ast = make_ast(Exp::Array {
            typ: Symbol::from("FooType"),
            size: boxed_ast(Exp::Int(1)),
            init: boxed_ast(Exp::Int(2))
        });
        let mut type_env = initial_type_env();
        let value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TArray(
            Arc::new(TigerType::TInt(R::RW)),
            TypeId::new(),
        ));
        type_env.insert(Symbol::from("FooType"), Arc::<TigerType>::clone(&foo_type));
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == *foo_type => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(..) => panic!("array")
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn arrayexp_size_not_int() {
        let ast = make_ast(Exp::Array {
            typ: Symbol::from("FooType"),
            size: boxed_ast(Exp::String(String::from("perro"))),
            init: boxed_ast(Exp::Int(2))
        });
        let mut type_env = initial_type_env();
        let value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TArray(
            Arc::new(TigerType::TInt(R::RW)),
            TypeId::new(),
        ));
        type_env.insert(Symbol::from("FooType"), foo_type);
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NonIntegerSize(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn arrayexp_type_mismatch() {
        let ast = make_ast(Exp::Array {
            typ: Symbol::from("FooType"),
            size: boxed_ast(Exp::Int(1)),
            init: boxed_ast(Exp::String(String::from("perro")))
        });
        let mut type_env = initial_type_env();
        let value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TArray(
            Arc::new(TigerType::TInt(R::RW)),
            TypeId::new(),
        ));
        type_env.insert(Symbol::from("FooType"), foo_type);
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TypeMismatch(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn arrayexp_not_array_type() {
        let ast = make_ast(Exp::Array {
            typ: Symbol::from("FooType"),
            size: boxed_ast(Exp::Int(1)),
            init: boxed_ast(Exp::String(String::from("perro")))
        });
        let mut type_env = initial_type_env();
        let value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TInt(R::RW));
        type_env.insert(Symbol::from("FooType"), foo_type);
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NotArrayType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn arrayexp_undeclared_type() {
        let ast = make_ast(Exp::Array {
            typ: Symbol::from("FooType"),
            size: boxed_ast(Exp::Int(1)),
            init: boxed_ast(Exp::String(String::from("perro")))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}