use super::super::ast::tigerabs::*;
use super::tigerseman::*;
use super::super::ast::position::Pos;


fn field_type<'a>(fields: &[(Box<String>, &'a Tipo<'a>, u8)], symbol: &str) -> Option<Tipo<'a>> {
    for var in fields.iter() {
        if *var.0 == symbol {
            return Some(var.1.clone());
        }
    }
    None
}

pub fn typecheck_var<'a>(var: &Var, pos: Pos, type_env: &TypeEnviroment, value_env: &ValueEnviroment<'a>) -> Result<Tipo<'a>, TypeError> {
    match var {
        Var::SimpleVar(var_symbol) => match value_env.get(var_symbol) {
            Some(env_entry) => match env_entry {
                EnvEntry::Var {
                    ty: var_type,
                    ..
                } => Ok(var_type.clone()),
                _ => Err(TypeError::NotSimpleVar(pos)),
            },
            None => Err(TypeError::UndeclaredSimpleVar(pos))
        },
        Var::SubscriptVar(boxed_var, index) => {
            let subscript_var = boxed_var;
            match &**subscript_var {
                Var::SimpleVar(s) => match value_env.get(s) {
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
                Var::FieldVar(..) | Var::SubscriptVar(..) => Err(TypeError::NotSimpleVar(pos)),
            }
        },
        Var::FieldVar(subscript_var, field_symbol) =>match &**subscript_var {
            Var::SimpleVar(record_symbol) => match value_env.get(record_symbol) {
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
            Var::FieldVar(..) | Var::SubscriptVar(..) => Err(TypeError::NotSimpleVar(pos)),
        },
    }

}

pub fn typecheck<'a>(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment<'a>) -> Result<Tipo<'a>, TypeError> {
    use _Exp::*;

    match exp {
        Exp { node: VarExp(var), pos} => {
            typecheck_var(var, *pos, type_env, value_env)
        },
        _ => panic!("le llego algo nada que ver a varexp::tipar")
    }
}


pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}