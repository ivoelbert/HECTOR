use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) ->  Result<Tipo, TypeError> {
    use Tipo::*;
    use std::collections::HashMap;
    let tipar_fields = |args: Vec<(Symbol, Box<Exp>)>| -> HashMap<Symbol, Result<Tipo, TypeError>> {
        return args.into_iter().map(|arg| (arg.0, tipar_exp(*arg.1, type_env.clone(), value_env.clone()))).rev().collect();
    };
    match exp { Exp {node: _Exp::RecordExp{fields, typ: record_type}, pos} => {
        let mut field_types = tipar_fields(fields);
        match type_env.get(&record_type) {
            Some(TRecord(formals, type_id)) => {
                for formal in formals {
                    match field_types.get(&*formal.0) {
                        Some(Ok(field_type)) => if *field_type == *formal.1 {
                            field_types.remove(&*formal.0);
                        }
                        else {
                            return Err(TypeError::TypeMismatch(pos));
                        },
                        Some(Err(type_error)) => return Err((*type_error).clone()),
                        None =>  return Err(TypeError::MissingRecordField(pos)),
                    }
                }
                if !field_types.is_empty() {
                    return Err(TypeError::TooManyArguments(pos));
                } else {
                    return Ok(TRecord((*formals).clone(), *type_id));
                }
            },
            Some(_) => return Err(TypeError::NotRecordType(pos)),
            None => return Err(TypeError::UndeclaredType(pos))
        }
    }
    _ => panic!("delegation panic on recordexp::tipar")
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}