// Arquitecture-specific details should be abstracted here.
// To make it easier to target diffent arquitectures, we could make Frame into a trait.

use super::{Label};
use crate::tree::*;

#[derive(Clone, Debug)]
pub struct Frame {
    name: String,
    formals: Vec<bool>,
    locals: Vec<bool>,
    actual_arg: i64,
    actual_local: i64,
    actual_reg: i64
}

static LOCAL_GAP: i64 = 4;

pub type Register = String;

#[derive(Clone, Debug)]
pub enum Access {
    InFrame(i64),
    InReg(Label)
}

#[derive(Clone)]
pub enum Frag {
    Proc {
        body: Tree::Stm,
        frame: Frame
    },
    ConstString(Label, String)
}

impl Frame {
    pub fn new(name: String, formals: Vec<bool>) -> Self {
        Frame {
            name,
            formals,
            locals: vec![],
            actual_arg: 0,
            actual_local: 0,
            actual_reg: 1,
        }
    }
    pub fn alloc_local(frame: &mut Self, escape: bool) -> Access {
        match escape {
            true => {
                let r = Access::InFrame(frame.actual_local + LOCAL_GAP);
                frame.actual_local = frame.actual_local -1;
                r
            }
            false => Access::InReg(newtemp())
        }
    }
    pub fn external_call(proc_name: String, args: Vec<Tree::Exp>) -> Tree::Exp {
        CALL(Box::new(NAME(proc_name)), args)
    }
}