use crate::ast::*;
use crate::tree::*;
use Tree::*;

pub fn translate(_exp: Exp) -> Result<Tree, TransError> {
    Ok(CONST(0))
}