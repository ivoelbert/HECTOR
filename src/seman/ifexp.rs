use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn typecheck<'a>(exp: &Exp<'a>, type_env: &'a TypeEnviroment<'a>, value_env: &ValueEnviroment<'a>) -> Result<Tipo<'a>, TypeError> {
    match exp { Exp {node: _Exp::IfExp{test, then_, else_}, pos} => {
        if !es_int(&tipo_real(type_exp(&*test, type_env, value_env)?, type_env)) {
            return Err(TypeError::NonIntegerCondition(*pos));
        }
        let then_type = type_exp(&*then_, type_env, value_env)?;
        match else_ {
            Some(else_exp) => match type_exp(&*else_exp, type_env, value_env) {
                Ok(else_type) => if else_type == then_type {
                    Ok(else_type)
                }
                else {
                    Err(TypeError::ThenElseTypeMismatch(*pos))
                }
                Err(type_error) => Err(type_error)
            }
            None => if then_type == Tipo::TUnit {
                Ok(Tipo::TUnit)
            } else {
                Err(TypeError::NonUnitBody(*pos))
            }
        }
    }
        _ => panic!("Delegation error on ifexp::tipar")
    }
}

pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}