use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    match node {
        Exp::Call {func: function_symbol, args,} => {
            let (formals, return_type) = match value_env.get(&function_symbol) {
                Some(EnvEntry::Func {
                    formals, result, ..
                }) => (formals, result),
                Some(EnvEntry::Var { .. }) => return Err(TypeError::NotFunctionVar(pos)),
                None => return Err(TypeError::UndeclaredFunction(pos)),
            };
            if formals.len() > args.len() {
                return Err(TypeError::TooFewArguments(pos));
            }
            if formals.len() < args.len() {
                return Err(TypeError::TooManyArguments(pos));
            }
            let typed_args : Vec<AST> = args
                .into_iter() // into_iter moves the values instead of borrowing
                .map(|arg| -> Result<AST, TypeError> {
                    type_exp(arg, type_env, value_env)
                })
                .collect::<Result<Vec<AST>, TypeError>>()?;
            // If any argument is not of it's formal type, fail.
            if formals.iter()
                .zip(typed_args.iter())
                .any(|(formal, typed_arg)|**formal != *typed_arg.typ) {
                return Err(TypeError::InvalidCallArgument(pos));
            };
            Ok(AST {
                node: Exp::Call {
                    func: function_symbol,
                    args: typed_args
                },
                pos,
                typ: return_type.clone()
            })
        }
        _ => panic!("delegation error"),
    }
}
#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn callexp_ok() {
        let ast = make_ast(Exp::Call {
            func: Symbol::from("f"),
            args: vec![],
        });
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        value_env.insert(Symbol::from("f"), EnvEntry::Func {
            formals: vec![],
            result: Arc::new(TigerType::TUnit),
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn callexp_too_many_args() {
        let ast = make_ast(Exp::Call {
            func: Symbol::from("f"),
            args: vec![make_ast(Exp::Int(1))],
        });
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        value_env.insert(Symbol::from("f"), EnvEntry::Func {
            formals: vec![],
            result: Arc::new(TigerType::TUnit),
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TooManyArguments(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn callexp_too_few_args() {
        let ast = make_ast(Exp::Call {
            func: Symbol::from("f"),
            args: vec![],
        });
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        value_env.insert(Symbol::from("f"), EnvEntry::Func {
            formals: vec![Arc::new(TigerType::TInt(R::RW))],
            result: Arc::new(TigerType::TUnit),
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TooFewArguments(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn callexp_undeclared_function() {
        let ast = make_ast(Exp::Call {
            func: Symbol::from("f"),
            args: vec![],
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredFunction(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}
