use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(_exp: Exp, _type_env: TypeEnviroment, _value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    return Ok(Tipo::TString);
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}