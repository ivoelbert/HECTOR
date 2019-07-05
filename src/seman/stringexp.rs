use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(_exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Tipo {
    return Tipo::TString;
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}