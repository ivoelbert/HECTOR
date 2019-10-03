use crate::ast::*;
use super::*;
use Tree::*;

pub fn translate(_exp: Exp) -> Result<Tree, TransError> {
    Ok(CONST(0))
}