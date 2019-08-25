use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(_exp: &Exp, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    Ok(Tipo::TInt(R::RW))
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}