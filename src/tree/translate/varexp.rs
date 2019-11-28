use super::super::frame::STATIC_LINK_OFFSET;
use crate::ast::*;
use crate::tree::*;
use Tree::Exp::*;

pub fn generate_static_link(remaining_depth: i64) -> Tree::Exp {
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

// Generates an expression that evaluates to the memory direction of the variable
pub fn simplevar(access: Access, nesting_depth: i64, current_level: &Level) -> Tree::Exp {
    let delta_depth = current_level.nesting_depth - nesting_depth;
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
    Var{kind, typ, ..}: &Var,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match kind {
        VarKind::Simple(name) => {
            if let Some(EnvEntry::Var{access, depth}) = value_env.get(name) {
                Ok((
                    simplevar(*access, *depth, &level),
                    level,
                    frags
                ))
            } else {
                panic!("typechecking should handle this")
            }
        },
        VarKind::Subscript(array, index) => {
            // The array is the memory pointer to the start
            // sum it with index times data size
            // there's a runtime to fail nicely
            // or we can let the dev get a segfault
            let (array_exp, array_level, array_frags) = trans_var(array, level, value_env, breaks_stack, frags)?;
            let (index_exp, index_level, index_frags) = super::trans_exp(index, array_level, value_env, breaks_stack, array_frags)?;
            Ok((MEM(Box::new(plus!(
                array_exp,
                BINOP(MUL, Box::new(index_exp), Box::new(CONST(frame::WORD_SIZE)))
            ))), index_level, index_frags))
        },
        VarKind::Field(record, field) => {
            // Similar to array
            // but you use the record's field order as "index"
            let (record_exp, record_level, record_frags) = trans_var(record, level, value_env, breaks_stack, frags)?;
            let formals = if let TigerType::TRecord(formals, ..) = &**typ {
                formals.clone()
            } else {
                panic!("typechecking should handle this")
            };
            let (_, _, order) = formals
                .iter()
                .find(|(name, ..)| {name == field})
                .expect("typechecking should handle this");
            Ok((MEM(Box::new(plus!(
                record_exp,
                // optimization candidate
                CONST(*order * frame::WORD_SIZE)
            ))), record_level, record_frags))
        },
    }
}
