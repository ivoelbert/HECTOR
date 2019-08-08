use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp { Exp {node: _Exp::WhileExp {test, body}, pos} => 
        match tipar_exp(*body, type_env, value_env) {
            Ok(Tipo::TUnit) => match tipar_exp(*test, type_env, value_env) {
                Ok(test_type) => match test_type.real(type_env) {
                    Some(Tipo::TInt(R::RW)) => Ok(Tipo::TUnit),
                    Some(_) =>  Err(TypeError::NonIntegerCondition(pos)),
                    None => panic!("real type panic")
                },
                Err(type_error) => Err(type_error)
            },
            Ok(_) => Err(TypeError::NonUnitBody(pos)),
            Err(type_error) => Err(type_error)
        }
        _ => panic!("le llego cualquier cosa a whileexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}