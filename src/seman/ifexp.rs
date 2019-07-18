use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    // El ML lo hace mal. Devuelve siempre TUnit. Eso debe traer problemas despues.
    match exp { Exp {node: _Exp::IfExp{test, then_, else_}, pos} => {
        use Tipo::*;
        let _ = match tipar_exp(*test, type_env.clone(), value_env.clone()) {
            Ok(TInt(_)) => (),
            Ok(_) => return Err(TypeError::NonIntegerCondition(pos)),
            Err(type_error) => return Err(type_error)
        };
        let then_type = match tipar_exp(*then_, type_env.clone(), value_env.clone()) {
            Ok(t) => t,
            Err(type_error) => return Err(type_error)
        };
        match else_ {
            Some(else_exp) => match tipar_exp(*else_exp, type_env.clone(), value_env.clone()) {
                Ok(else_type) => if else_type == then_type {
                    return Ok(else_type);
                }
                else {
                    return Err(TypeError::ThenElseTypeMismatch(pos));
                }
                Err(type_error) => Err(type_error)
            }
            None => return Ok(TUnit)
        }
    }
        _ => panic!("Delegation error on ifexp::tipar")
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}