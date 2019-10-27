use super::super::frame::STATIC_LINK_OFFSET;
use crate::ast::*;
use crate::tree::*;
use Tree::Exp::*;

// Generates an expression that evaluates to the memory direction of the variable
pub fn simplevar(access: Access, nesting_depth: i64, top_level: &Level) -> Tree::Exp {
    fn generate_static_link(remaining_depth: i64) -> Tree::Exp {
        match remaining_depth {
            1 => MEM(Box::new(plus!(
                TEMP(Temp::FRAME_POINTER),
                CONST(STATIC_LINK_OFFSET)
            ))),
            d => MEM(Box::new(plus!(
                generate_static_link(d - 1),
                CONST(STATIC_LINK_OFFSET)
            ))),
        }
    }
    let delta_depth = top_level.nesting_depth - nesting_depth;
    match access {
        Access::InReg(t) => {
            if delta_depth == 0 {
                TEMP(t)
            } else {
                panic!("escaped InReg!")
            }
        }
        Access::InFrame(offset) => {
            // We assume all InFrame escape
            MEM(Box::new(plus!(
                generate_static_link(delta_depth),
                CONST(offset)
            )))
        }
    }
}

pub fn trans_var(
    var: &Var,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    //TODO
    match var {
        Var::Simple(name) => Ok((CONST(0), frags)),
        Var::Subscript(array, index) => Ok((CONST(0), frags)),
        Var::Field(record, field) => Ok((CONST(0), frags)),
    }
}
