use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp { Exp {node: _Exp::WhileExp {test, body}, pos} =>{
            if !es_int(&tipo_real(type_exp(&*test, type_env, value_env)?, type_env)) {
                return Err(TypeError::NonIntegerCondition(*pos));
            }
            match type_exp(body, type_env, value_env) {
                Ok(Tipo::TUnit) => Ok(Tipo::TUnit),
                Ok(_) => Err(TypeError::NonUnitBody(*pos)),
                Err(type_error) => Err(type_error)
            }
        }
        _ => panic!("le llego cualquier cosa a whileexp::tipar")
    }
}

pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}