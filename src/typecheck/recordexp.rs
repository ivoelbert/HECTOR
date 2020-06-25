use crate::ast::*;
use crate::typecheck::*;

/// Rebuild an `Exp::Record` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    let type_field_inits = |args: Vec<(Symbol, Box<AST>)>| -> Result<HashMap<Symbol, AST>, TypeError> {
        args
            .into_iter()
            .map(|(symbol, ast)| -> Result<(Symbol, AST), TypeError> {
                Ok((symbol, type_exp(*ast, type_env, value_env)?))
            })
            .collect()
    };
    match node {
        Exp::Record{fields, typ: record_type_symbol} => {
            // Type the fields
            // If the record type is not a record type, error.
            // If some field doesn't match the formal type, error.
            // if field <-> formals is not a 1:1, error.
            let mut typed_field_inits = type_field_inits(fields)?;
            let record_type = if let Some(tipo) = type_env.get(&record_type_symbol) {
                tipo_real(Arc::clone(&tipo), type_env)
            } else {
                console_log!("arrayexp undeclared");
                return Err(TypeError::UndeclaredType(pos))
            };
            match &*record_type {
                TigerType::TRecord(formals, type_id) => {
                    if typed_field_inits.len() > formals.len() {
                        return Err(TypeError::TooManyArguments(pos))
                    };
                    if typed_field_inits.len() < formals.len() {
                        return Err(TypeError::MissingRecordField(pos))
                    };
                    Ok(AST {
                        node: Exp::Record{
                            fields: formals
                                .iter()
                                .map(|(name, typ, ..)| -> Result<(Symbol, Box<AST>), TypeError> {
                                    match typed_field_inits.remove(name) {
                                        Some(ast) => {
                                            if RecordFieldType::Type(Arc::clone(&ast.typ)) == *typ {
                                                Ok((name.clone(), Box::new(ast)))
                                            } else {
                                                console_log!("record mismatch");
                                                Err(TypeError::TypeMismatch(pos))
                                            }
                                        }
                                        None => Err(TypeError::MissingRecordField(pos))
                                    }
                                })
                                .collect::<Result<Vec<(Symbol, Box<AST>)>, TypeError>>()?,
                            typ: record_type_symbol,
                        },
                        typ: Arc::new(TigerType::TRecord(formals.clone(), *type_id)),
                        pos
                    })
                },
                _ => Err(TypeError::NotRecordType(pos)),
            }
        }
        _ => panic!("delegation panic on recordexp::tipar")
    }
}
#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;

    #[test]
    #[wasm_bindgen_test]
    fn recordexp_ok() {
        let ast = make_ast(Exp::Record {
            fields: vec![(Symbol::from("baz"), boxed_ast(Exp::Int(1)))],
            typ: Symbol::from("FooType"),
        });
        let mut type_env = initial_type_env();
        let value_env = initial_value_env();
        let field_type = Arc::new(TigerType::TInt(R::RW));
        let foo_type = Arc::new(TigerType::TRecord(
                vec![(String::from("baz"),
                    RecordFieldType::Type(field_type),
                    0)], TypeId::new()));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == *foo_type => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn recordexp_undeclard_type() {
        let ast = make_ast(Exp::Record {
            fields: vec![(Symbol::from("baz"), boxed_ast(Exp::Int(1)))],
            typ: Symbol::from("FooType"),
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

    #[test]
    #[wasm_bindgen_test]
    fn recordexp_non_record_type() {
        let ast = make_ast(Exp::Record {
            fields: vec![(Symbol::from("baz"), boxed_ast(Exp::Int(1)))],
            typ: Symbol::from("FooType"),
        });
        let mut type_env = initial_type_env();
        let value_env = initial_value_env();
        type_env.insert(Symbol::from("FooType"), Arc::new(TigerType::TInt(R::RW)));
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NotRecordType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}