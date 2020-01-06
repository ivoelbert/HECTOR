// This modules abtract arquitecture specific concepts during translation phase.
// A frame is wrapped in a level as it's being built and when finished it's stored in a fragment
// Registers are wrapped in temporaries.

extern crate nanoid;

pub use super::frame::{Frame};
use super::Access;
pub type Label = String;
type LocalTemp = String;
use serde::{Serialize, Serializer};


#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Temp {
    FRAME_POINTER,
    RV, // Return Value
    // Other special temps.
    Local(LocalTemp),
}

pub fn newtemp() -> Temp {
    Temp::Local(nanoid::simple())
}

pub fn newlabel() -> Label {
    nanoid::simple()
}


#[derive(Clone, Debug, Serialize)]
pub struct Level {
    pub frame: Frame,
    pub nesting_depth: i64,
}

impl Level {
    pub fn outermost() -> Level {
        Level {
            frame: Frame::new(
                newlabel(),
                vec![],
            ),
            nesting_depth: -1,
        }
    }

    pub fn new(depth: i64, name: Label, formals: Vec<bool>) -> Level {
        Level {
            frame: Frame::new(name, formals),
            nesting_depth: depth,
        }
    }

    pub fn alloc_arg(self: &mut Self, escape: bool) -> Access {
        self.frame.alloc_arg(escape)
    }

    pub fn alloc_local(self: &mut Self, escape: bool)  -> Access {
        self.frame.alloc_local(escape)
    }
}