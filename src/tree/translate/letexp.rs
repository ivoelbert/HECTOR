use crate::ast::*;
use crate::tree::*;
use ExpInterm::*;
use Tree::Exp::*;
use Tree::Stm::*;

pub fn translate(_exp: Exp) -> Result<ExpInterm, TransError> {
    Ok(Ex(CONST(0)))
}