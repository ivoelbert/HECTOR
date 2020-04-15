// This modules abtract arquitecture specific concepts during translation phase.
// A frame is wrapped in a level as it's being built and when finished it's stored in a fragment

extern crate uuid;

use super::frame::Frame;
pub use super::frame::{LocalTemp, newlocal, unique_named_local,
    FRAME_POINTER, STACK_POINTER, RETURN_VALUE, STATIC_LINK_OFFSET, external_call};
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
    name.to_string()
}

pub fn unique_named_label(name: &str) -> Label {
    vec![name.to_string(), "_".to_string(), Uuid::new_v4().to_string()].concat()
}

pub fn newglobal() -> GlobalTemp {
    Uuid::new_v4().to_string()
}

pub fn unique_named_global(name: &str) -> Label {
    vec![name.to_string(), "_".to_string(), Uuid::new_v4().to_string()].concat()
}

pub fn named_global(name: &str) -> GlobalTemp {
    String::from(name)
}

#[derive(Clone, Serialize)]
pub enum Fragment {
    Proc {
        body: Tree::Stm,
        frame: Frame
    },
    ConstString(Label, String)
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
                named_label("outermost"),
            ),
            nesting_depth: -1,
        }
    }

    pub fn new(depth: i32, label: Label) -> Level {
        Level {
            frame: Frame::new(label),
            nesting_depth: depth,
        }
    }

    pub fn alloc_arg(self: &mut Self, name: String, escape: bool) -> Access {
        self.frame.alloc_arg(name, escape)
    }

    pub fn alloc_local(self: &mut Self, escape: bool, name: Option<String>) -> Access {
        self.frame.alloc_local(escape, name)
    }

    pub fn access_to_exp(self: &Self, access: Access, depth: i32) -> Tree::Exp {
        use Tree::{Exp::*, BinOp::*};
        fn search_correct_frame_pointer(remaining_depth: i32) -> Tree::Exp {
            if remaining_depth < 1 {panic!("depth: {:?}", remaining_depth)};
            match remaining_depth {
                1 => MEM(Box::new(plus!(
                    GLOBAL(named_global(FRAME_POINTER)),
                    CONST(STATIC_LINK_OFFSET)
                ))),
                d => MEM(Box::new(plus!(
                    search_correct_frame_pointer(d - 1),
                    CONST(STATIC_LINK_OFFSET)
                ))),
            }
        }
        let delta_depth = self.nesting_depth - depth;
        match access {
            Access::InMem(i) => {
                if delta_depth == 0 {
                    MEM(Box::new(BINOP(PLUS,
                        Box::new(GLOBAL(named_global(FRAME_POINTER))),
                        Box::new(CONST(i))
                    )))
                } else {
                    let sl = search_correct_frame_pointer(delta_depth);
                    MEM(Box::new(BINOP(PLUS,
                        Box::new(sl),
                        Box::new(CONST(i))
                    )))
                }
            },
            Access::InGlobal(l) => GLOBAL(l),
            Access::InLocal(l) => {
                if delta_depth != 0 {
                    panic!("escaped local!")
                }
                LOCAL(l)
            }
        }
    }

    pub fn finish(self: Self, body: Tree::Stm) -> Fragment {
        Fragment::Proc{
            body: Tree::Stm::SEQ(
                Box::new(self.frame.generate_move_escaped_arguments_statement()),
                Box::new(body)
            ),
            frame: self.frame
        }
    }
}