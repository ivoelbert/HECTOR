use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp) -> Tipo {
    return Tipo::TUnit;
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}