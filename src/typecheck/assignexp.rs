use super::*;

pub fn typecheck(
    ast: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    use TigerType::*;
    use super::varexp::typecheck_var;
    match ast {
        AST {node: Exp::Assign{var, exp: value_exp}, pos, ..} => {
            let typed_var = typecheck_var(var, type_env, value_env)?;
            let value_ast = type_exp(*value_exp, type_env, value_env)?;
            let var_type = match &*typed_var.typ {
                TInt(R::RO) => return Err(TypeError::ReadOnlyAssignment(pos)),
                tiger_type => tiger_type.clone(),
            };
            if var_type != *value_ast.typ {
                console_log!("assign mismatch");
                return Err(TypeError::TypeMismatch(pos))
            }
            Ok(AST {
                node: Exp::Assign{
                    var: typed_var,
                    exp: Box::new(value_ast)
                },
                pos,
                typ: Arc::new(TUnit)
            })
        },
        _ => panic!("Mala delegacion en seman")
    }
}


#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn assignexp_ok() {
        let ast = make_ast(Exp::Assign{
            var: make_var(VarKind::Simple(Symbol::from("foo"))),
            exp: boxed_ast(Exp::Int(1)),
        });
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let env_entry = EnvEntry::Var{
            ty: Arc::new(TigerType::TInt(R::RW)),
        };
        value_env.insert(Symbol::from("foo"), env_entry);
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn assignexp_undeclared_variable() {
        let ast = make_ast(Exp::Assign{
            var: make_var(VarKind::Simple(Symbol::from("foo"))),
            exp: boxed_ast(Exp::Int(1)),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredSimpleVar(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn assignexp_type_mismatch() {
        let ast = make_ast(Exp::Assign{
            var: make_var(VarKind::Simple(Symbol::from("foo"))),
            exp: boxed_ast(Exp::String(String::from("perro"))),
        });
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let env_entry = EnvEntry::Var{
            ty: Arc::new(TigerType::TInt(R::RW)),
        };
        value_env.insert(Symbol::from("foo"), env_entry);
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TypeMismatch(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn assignexp_variable_read_only() {
        let ast = make_ast(Exp::Assign{
            var: make_var(VarKind::Simple(Symbol::from("i"))),
            exp: boxed_ast(Exp::Int(2)),
        });
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let env_entry = EnvEntry::Var{
            ty: Arc::new(TigerType::TInt(R::RO)),
        };
        value_env.insert(Symbol::from("i"), env_entry);
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::ReadOnlyAssignment(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}