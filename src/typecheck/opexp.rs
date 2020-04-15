use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment)
-> Result<AST, TypeError> {
    match node {
        Exp::Op{left, right, oper} => {
            let left_ast = type_exp(*left, type_env, value_env)?;
            let right_ast = type_exp(*right, type_env, value_env)?;
            let left_type = tipo_real(left_ast.typ.clone(), type_env);
            let right_type = tipo_real(right_ast.typ.clone(), type_env);
            let op_ast = AST {
                node: Exp::Op {
                    oper,
                    left: Box::new(left_ast),
                    right: Box::new(right_ast),
                },
                pos,
                typ: Arc::new(TigerType::TInt(R::RW))
            };
            match oper {
                Oper::EqOp | Oper::NeqOp => {
                    if left_type == right_type {
                        Ok(op_ast)
                    } else {
                        console_log!("op eq mismatch. node: {:?}, lt: {:?}, rt: {:?}", &op_ast, &left_type, &right_type);
                        Err(TypeError::TypeMismatch(pos))
                    }
                },
                Oper::PlusOp | Oper::MinusOp | Oper::TimesOp | Oper::DivideOp => {
                    if es_int(&left_type) && es_int(&right_type)  {
                        Ok(op_ast)
                    } else {
                        console_log!("op plus mismatch");
                        Err(TypeError::TypeMismatch(pos))
                    }
                },
                Oper::LtOp | Oper::LeOp | Oper::GtOp | Oper::GeOp => {
                    if (es_int(&left_type) && es_int(&right_type)) || (*left_type == TigerType::TString && *right_type == TigerType::TString) {
                        Ok(op_ast)
                    } else {
                        console_log!("op lt mismatch");
                        Err(TypeError::TypeMismatch(pos))
                    }
                }
            }
        }
        _ => panic!("delegation errror on opexp::tipar")
    }
}
#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;

    #[test]
    #[wasm_bindgen_test]
    fn opexp_ok() {
        let ast = make_ast(Exp::Op {
            left: boxed_ast(Exp::Int(1)),
            oper: Oper::PlusOp,
            right: boxed_ast(Exp::Int(1)),
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
    fn opexp_type_mismatch() {
        let ast = make_ast(Exp::Op {
            left: boxed_ast(Exp::Int(1)),
            oper: Oper::PlusOp,
            right: boxed_ast(Exp::String(String::from("perro"))),
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TypeMismatch(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}