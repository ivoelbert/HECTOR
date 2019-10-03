use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<TigerType, TypeError> {
    match exp { Exp {node: _Exp::While {test, body}, pos} =>{
            if !es_int(&tipo_real(type_exp(&*test, type_env, value_env)?, type_env)) {
                return Err(TypeError::NonIntegerCondition(*pos));
            }
            match type_exp(body, type_env, value_env) {
                Ok(TigerType::TUnit) => Ok(TigerType::TUnit),
                Ok(_) => Err(TypeError::NonUnitBody(*pos)),
                Err(type_error) => Err(type_error)
            }
        }
        _ => panic!("le llego cualquier cosa a whileexp::tipar")
    }
}

