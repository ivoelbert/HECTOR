use super::super::ast::tigerabs::*;
use super::tigerseman::*;
use super::super::ast::position::Pos;


fn field_type(fields: &[(Box<String>, Box<Tipo>, u8)], symbol: &str) -> Option<Tipo> {
    for var in fields.iter() {
        if *var.0 == symbol {
            return Some(*var.1.clone());
        }
    }
    None
}

pub fn tipar_var(var: Var, pos: Pos, type_env: TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    use Var::*;
    use Tipo::*;
    match var {
        SimpleVar(var_symbol) => match value_env.get(&var_symbol) {
            Some(env_entry) => match env_entry {
                EnvEntry::Var {
                    ty: var_type,
                    ..
                } => Ok(var_type.clone()),
                _ => Err(TypeError::NotSimpleVar(pos)),
            },
            None => Err(TypeError::UndeclaredSimpleVar(pos))
        },
        SubscriptVar(var, index) => match *var {
            SimpleVar(s) => match value_env.get(&s) {
                    Some(env_entry) => match env_entry {
                        EnvEntry::Var {
                            ty: TArray(array_of, _),
                            ..
                        } => match tipar_exp(*index, type_env, value_env.clone()){
                                Ok(TInt(_)) => Ok(*array_of.clone()),
                                Ok(_) => Err(TypeError::SunscriptNotInteger(pos)),
                                Err(e) => Err(e)
                            },
                        _ => Err(TypeError::NotArrayType(pos))
                    }, 
                    None => Err(TypeError::UndeclaredSimpleVar(pos))
            },
            FieldVar(..) | SubscriptVar(..) => Err(TypeError::NotSimpleVar(pos)),
        },
        FieldVar(var, field_symbol) =>match *var {
            SimpleVar(record_symbol) => match value_env.get(&record_symbol) {
                Some(env_entry) => match env_entry {
                    EnvEntry::Var {ty: TRecord(vars, _), ..}
                        => match field_type(vars, &field_symbol) {
                            Some(field_type) => Ok(field_type),
                            None => Err(TypeError::FieldDoesNotExist(pos))
                        }
                    _ => Err(TypeError::NotRecordType(pos))
                }, 
                None => Err(TypeError::UndeclaredSimpleVar(pos))
            },
            FieldVar(..) | SubscriptVar(..) => Err(TypeError::NotSimpleVar(pos)),
        },
    }

}

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    use _Exp::*;

    match exp {
        Exp { node: VarExp(var), pos} => tipar_var(var, pos, type_env, &value_env), 
        _ => panic!("le llego algo nada que ver a varexp::tipar")
    }
}


pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}