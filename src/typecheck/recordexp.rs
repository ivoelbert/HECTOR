use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) ->  Result<Tipo, TypeError> {
    let tipar_fields = |args: &Vec<(Symbol, Box<Exp>)>| -> HashMap<Symbol, Result<Tipo, TypeError>> {
        args.iter().map(|arg| (arg.0.clone(), type_exp(&*arg.1, type_env, value_env))).rev().collect()
    };
    match exp { Exp {node: _Exp::Record{fields, typ: record_type_string, ..}, pos} => {
        let mut field_types = tipar_fields(fields);

        let record_type = match type_env.get(record_type_string) {
            Some(tipo) => tipo_real(tipo.clone(), type_env),
            None => return Err(TypeError::UndeclaredType(*pos))
        };
        match record_type {
            Tipo::TRecord(formals, type_id) => {
                for formal in formals.clone() {
                    match field_types.get(&*formal.0) {
                        Some(Ok(field_type)) => if *field_type == *formal.1 {
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
                    Ok(Tipo::TRecord(formals, type_id))
                } else {
                    Err(TypeError::TooManyArguments(*pos))
                }
            },
            Tipo::TUnit | Tipo::TNil | Tipo::TInt(..) | Tipo::TString | Tipo::TArray(..) | Tipo::TipoInterno(..) => Err(TypeError::NotRecordType(*pos)),
        }
    }
    _ => panic!("delegation panic on recordexp::tipar")
    }
}
