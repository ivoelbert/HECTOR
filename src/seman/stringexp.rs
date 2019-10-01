use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn typecheck(_exp: &Exp, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    Ok(Tipo::TString)
}

pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}