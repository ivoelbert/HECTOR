use crate::ast::*;
use crate::tree::*;
use ExpInterm::*;
use Tree::Exp::*;

pub fn translate(_exp: Exp) -> Result<ExpInterm, TransError> {
    Ok(Ex(CONST(0)))
}