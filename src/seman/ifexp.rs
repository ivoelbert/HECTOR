use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp { Exp {node: _Exp::IfExp{test, then_, else_}, pos} => {
        use Tipo::*;
        match tipar_exp(&*test, type_env, value_env) {
            Ok(TInt(_)) => (),
            Ok(_) => return Err(TypeError::NonIntegerCondition(*pos)),
            Err(type_error) => return Err(type_error)
        };
        let then_type = tipar_exp(&*then_, type_env, value_env)?;
        match else_ {
            Some(else_exp) => match tipar_exp(&*else_exp, type_env, value_env) {
                Ok(else_type) => if else_type == then_type {
                    Ok(else_type)
                }
                else {
                    Err(TypeError::ThenElseTypeMismatch(*pos))
                }
                Err(type_error) => Err(type_error)
            }
            None => if then_type == TUnit {
                Ok(TUnit)
            } else {
                Err(TypeError::NonUnitBody(*pos))
            }
        }
    }
        _ => panic!("Delegation error on ifexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}