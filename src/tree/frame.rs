// Arquitecture-specific details should be abstracted here.
// To make it easier to target diffent arquitectures, we could make Frame into a trait.

use super::{Label};
use crate::tree::*;
use serde::{Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Frame {
    name: String,
    label: Label,
    formals: Vec<bool>,
    locals: Vec<bool>,
    actual_arg: i64,
    actual_local: i64,
    actual_reg: i64
}

// TODO: all of this constants
pub static LOCAL_GAP: i64 = 4;
pub static STATIC_LINK_OFFSET: i64 = 1337;
pub static WORD_SIZE: i64 = 4;

#[derive(Clone, Debug, Serialize)]
pub enum Access {
    InFrame(i64),
    InReg(Temp)
}

#[derive(Clone, Serialize)]
pub enum Frag {
    Proc {
        body: Tree::Stm,
        frame: Frame
    },
    ConstString(Label, String)
}

impl Frag {
    pub fn new(tree: Tree::Stm, level: Level) -> Self {
        Frag::Proc {
            body: tree,
            frame: level.frame
        }
    }
}

impl Frame {
    pub fn new(name: String, label: Label, formals: Vec<bool>) -> Self {
        Frame {
            name,
            label,
            formals,
            locals: vec![],
            actual_arg: 0,
            actual_local: 0,
            actual_reg: 1,
        }
    }

    pub fn alloc_local(self: &mut Self, escape: bool) -> Access {
        match escape {
            true => {
                let r = Access::InFrame(self.actual_local + LOCAL_GAP);
                self.actual_local = self.actual_local -1;
                r
            }
            false => Access::InReg(newtemp())
        }
    }

    pub fn alloc_arg(self: &mut Self, escape: bool) -> Access {
        match escape {
            true => {
                let r = Access::InFrame(self.actual_arg + LOCAL_GAP);
                self.actual_arg = self.actual_arg -1;
                r
            }
            false => Access::InReg(newtemp())
        }
    }

    // abstrae la llamada al runtime
    pub fn external_call(proc_name: String, proc_label: Label, args: Vec<Tree::Exp>) -> Tree::Exp {
        CALL(proc_name, Box::new(NAME(proc_label)), args)
    }

    // la funcion formals va en typescript

    // TODO: prologo y epilogo de funcion
    // label de funcion
    // guardar registros callee-saved
    // o no hace NADA?
    // pub fn add_function_prologue_epilogue()
}