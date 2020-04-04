// This modules abtract arquitecture specific concepts during translation phase.
// A frame is wrapped in a level as it's being built and when finished it's stored in a fragment
// Registers are wrapped in temporaries.

extern crate uuid;

pub use super::frame::{Frame, LocalTemp, newlocal, named_local, FRAME_POINTER, STACK_POINTER, RETURN_VALUE, external_call};
use super::Access;
use super::Tree;

pub type Label = String;
pub type GlobalTemp = String;

use serde::{Serialize};
use uuid::Uuid;

pub fn newlabel() -> Label {
    Uuid::new_v4().to_string()
}

pub fn named_label(name: &str) -> Label {
    String::from(name)
}

pub fn newglobal() -> GlobalTemp {
    Uuid::new_v4().to_string()
}

pub fn named_global(name: &str) -> GlobalTemp {
    String::from(name)
}

#[derive(Clone, Debug, Serialize)]
pub struct Level {
    pub frame: Frame,
    pub nesting_depth: i32,
}

impl Level {
    pub fn outermost() -> Level {
        Level {
            frame: Frame::new(
                String::from("outermost"),
                newlabel(),
            ),
            nesting_depth: -1,
        }
    }

    pub fn new(depth: i32, name: String, label: Label) -> Level {
        Level {
            frame: Frame::new(name, label),
            nesting_depth: depth,
        }
    }

    pub fn alloc_arg(self: &mut Self, name: String, escape: bool) -> Access {
        self.frame.alloc_arg(name, escape)
    }

    pub fn alloc_local(self: &mut Self, escape: bool, name: Option<String>) -> Access {
        self.frame.alloc_local(escape, name)
    }

    pub fn access_to_exp(self: &Self, access: Access) -> Tree::Exp {
        use Tree::Exp::*;
        match access {
            Access::InMem(i) => MEM(Box::new(CONST(i))),
            Access::InGlobal(l) => GLOBAL(l),
            Access::InLocal(l) => LOCAL(l)
        }
    }
}