use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn typecheck<'a>(_exp: &Exp, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<Tipo<'a>, TypeError> {
    Ok(Tipo::TUnit)
}

pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}