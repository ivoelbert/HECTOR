use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar<'a>(_exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo<'a>, TypeError> {
    return Ok(Tipo::TUnit);
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}