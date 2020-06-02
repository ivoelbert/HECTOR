use super::*;

/// Rebuild an `Exp::For` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env:& ValueEnviroment
) -> Result<AST, TypeError> {
    match node {
        Exp::For {var, lo, hi, body, escape} => {
            let lo_ast = type_exp(*lo, type_env, value_env)?;
            let hi_ast = type_exp(*hi, type_env, value_env)?;
            let lo_type = tipo_real(Arc::clone(&lo_ast.typ), type_env);
            let hi_type = tipo_real(Arc::clone(&hi_ast.typ), type_env);
            if !es_int(&lo_type) || !es_int(&hi_type) {
                return Err(TypeError::NonIntegerForRange(pos));
            }
            let mut new_value_env = value_env.clone();
            new_value_env.insert(var.clone(), EnvEntry::Var {
                ty: Arc::new(TigerType::TInt(R::RO)),
            });
            let body_ast = type_exp(*body, type_env, &new_value_env)?;
            if *body_ast.typ != TigerType::TUnit {
                return Err(TypeError::NonUnitBody(pos));
            }
            Ok(AST {
                node: Exp::For {
                    var,
                    lo: Box::new(lo_ast),
                    hi: Box::new(hi_ast),
                    body: Box::new(body_ast),
                    escape
                },
                pos,
                typ: Arc::new(TigerType::TUnit)
            })
        }
        _ => panic!("delegation panic in forexp::tipar")
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn forexp_ok() {
        let ast = make_ast(Exp::For {
            var: Symbol::from("i"),
            escape: false,
            lo: boxed_ast(Exp::Int(1)),
            hi: boxed_ast(Exp::Int(10)),
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
    fn forexp_iterator_is_usable() {
        let ast = make_ast(Exp::For {
            var: Symbol::from("i"),
            escape: false,
            lo: boxed_ast(Exp::Int(1)),
            hi: boxed_ast(Exp::Int(10)),
            body: boxed_ast(Exp::Seq(vec![
                make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("i"))))),
                make_ast(Exp::Unit)])),
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
    fn forexp_npn_unit_body() {
        let ast = make_ast(Exp::For {
            var: Symbol::from("i"),
            escape: false,
            lo: boxed_ast(Exp::Int(1)),
            hi: boxed_ast(Exp::Int(10)),
            body: boxed_ast(Exp::Int(2)),
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

    #[test]
    #[wasm_bindgen_test]
    fn forexp_non_unit_lo() {
        let ast = make_ast(Exp::For {
            var: Symbol::from("i"),
            escape: false,
            lo: boxed_ast(Exp::Unit),
            hi: boxed_ast(Exp::Int(10)),
            body: boxed_ast(Exp::Unit),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NonIntegerForRange(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn forexp_non_unit_hi() {
        let ast = make_ast(Exp::For {
            var: Symbol::from("i"),
            escape: false,
            lo: boxed_ast(Exp::Int(1)),
            hi: boxed_ast(Exp::Unit),
            body: boxed_ast(Exp::Unit),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NonIntegerForRange(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}