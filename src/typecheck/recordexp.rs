use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Arc<TigerType>, TypeError> {
    let tipar_fields = |args: &Vec<(Symbol, Box<Exp>)>| -> HashMap<Symbol, Result<Arc<TigerType>, TypeError>> {
        args.iter().map(|arg| (arg.0.clone(), type_exp(&*arg.1, type_env, value_env))).rev().collect()
    };
    match exp { Exp {node: _Exp::Record{fields, typ: record_type_string, ..}, pos} => {
        let mut field_types = tipar_fields(fields);

        let record_type = &*match type_env.get(record_type_string) {
            Some(tipo) => tipo_real((*tipo).clone(), type_env).clone(),
            None => return Err(TypeError::UndeclaredType(*pos))
        };
        match record_type {
            TigerType::TRecord(formals, type_id) => {
                for formal in formals {
                    match field_types.get(&*formal.0) {
                        Some(Ok(field_type)) => if *field_type == formal.1 {
                            field_types.remove(&*formal.0);
                        }
                        else {
                            return Err(TypeError::TypeMismatch(*pos));
                        },
                        Some(Err(type_error)) => return Err((*type_error).clone()),
                        None =>  return Err(TypeError::MissingRecordField(*pos)),
                    }
                }
                if field_types.is_empty() {
                    Ok(Arc::new(TigerType::TRecord(formals.clone(), *type_id)))
                } else {
                    Err(TypeError::TooManyArguments(*pos))
                }
            },
            TigerType::TUnit | TigerType::TNil | TigerType::TInt(..) | TigerType::TString | TigerType::TArray(..) | TigerType::Internal(..) => Err(TypeError::NotRecordType(*pos)),
        }
    }
    _ => panic!("delegation panic on recordexp::tipar")
    }
}
