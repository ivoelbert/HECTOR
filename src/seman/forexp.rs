use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: &Exp, type_env: &TypeEnviroment, value_env:& ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    match exp { Exp {node: _Exp::ForExp {var, lo, hi, body, ..}, pos} => {
        match tipar_exp(&*lo, type_env, value_env) {
            Ok(TInt(_)) => (),
            Ok(_) => return Err(TypeError::NonIntegerForRange(*pos)),
            Err(type_error) => return Err(type_error)
        };
        match tipar_exp(&*hi, type_env, value_env) {
            Ok(TInt(_)) => (),
            Ok(_) => return Err(TypeError::NonIntegerForRange(*pos)),
            Err(type_error) => return Err(type_error)
        };
        let mut new_value_env = value_env.clone();
        new_value_env.insert(var.clone(), EnvEntry::Var {
            ty: TInt(R::RO),
            access: Access::InFrame(0),
            level: 0
        });
        match tipar_exp(&*body, type_env, &new_value_env) {
            Ok(TUnit) => (),
            Ok(_) => return Err(TypeError::NonUnitBody(*pos)),
            Err(type_error) => return Err(type_error)
        };
        Ok(TUnit)
    }
    _ => panic!("delegation panic in forexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}