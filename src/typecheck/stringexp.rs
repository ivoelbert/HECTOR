use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(_exp: &Exp, _type_env: &TypeEnviroment, _value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    Ok(Tipo::TString)
}
