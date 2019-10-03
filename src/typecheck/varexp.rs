use crate::ast::*;
use crate::typecheck::*;

fn field_type(fields: &[(Box<String>, Box<Tipo>, u8)], symbol: &str) -> Option<Tipo> {
    for var in fields {
        if *var.0 == symbol {
            return Some(*var.1.clone());
        }
    }
    None
}

pub fn typecheck_var(var: &Var, pos: Pos, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match var {
        Var::Simple(var_symbol) => match value_env.get(var_symbol) {
            Some(env_entry) => match env_entry {
                EnvEntry::Var {
                    ty: var_type,
                    ..
                } => Ok(var_type.clone()),
                _ => Err(TypeError::NotSimpleVar(pos)),
            },
            None => Err(TypeError::UndeclaredSimpleVar(pos))
        },
        Var::Subscript(boxed_var, index) => {
            let subscript_var = boxed_var;
            match &**subscript_var {
                Var::Simple(s) => match value_env.get(s) {
                    Some(env_entry) => match env_entry {
                        EnvEntry::Var {
                            ty: Tipo::TArray(array_of, _),
                            ..
                        } => match type_exp(&*index, type_env, value_env){
                                Ok(Tipo::TInt(_)) => Ok(*array_of.clone()),
                                Ok(_) => Err(TypeError::SunscriptNotInteger(pos)),
                                Err(e) => Err(e)
                            },
                        _ => Err(TypeError::NotArrayType(pos))
                    },
                    None => Err(TypeError::UndeclaredSimpleVar(pos))
                }
                Var::Field(..) | Var::Subscript(..) => Err(TypeError::NotSimpleVar(pos)),
            }
        },
        Var::Field(subscript_var, field_symbol) =>match &**subscript_var {
            Var::Simple(record_symbol) => match value_env.get(record_symbol) {
                Some(env_entry) => match env_entry {
                    EnvEntry::Var {ty: Tipo::TRecord(vars, _), ..}
                        => match field_type(vars, &field_symbol) {
                            Some(field_type) => Ok(field_type),
                            None => Err(TypeError::FieldDoesNotExist(pos))
                        }
                    _ => Err(TypeError::NotRecordType(pos))
                },
                None => Err(TypeError::UndeclaredSimpleVar(pos))
            },
            Var::Field(..) | Var::Subscript(..) => Err(TypeError::NotSimpleVar(pos)),
        },
    }

}

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp {
        Exp { node: _Exp::Var(var), pos} => {
            typecheck_var(var, *pos, type_env, value_env)
        },
        _ => panic!("le llego algo nada que ver a varexp::tipar")
    }
}
