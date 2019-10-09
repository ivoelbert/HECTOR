
pub mod escape;
pub mod frame;
#[allow(non_snake_case)]
pub mod Tree;
mod translate;
pub mod temp;

pub use temp::*;
pub use frame::{Frame};

pub enum ExpInterm {
    Ex(Tree::Exp),
    Nx(Tree::Stm),
    Cx(Box<dyn Fn(Label, Label) -> Tree::Stm>)
}
pub enum TransError {
    BreakError,
    DivByZero
}

static mut ACTUAL_LEVEL : i64 = 0;

fn outermost() -> Level {
    Level {
        parent: None,
        frame: Frame::new(
            Label::from("_tigermain"),
            vec![],
        ),
        nesting_level: -1
    }
}

fn new_level(parent_level: Level, name: Label, formals: Vec<bool>) -> Level {
    Level {
        parent: Some(parent_level.frame),
        frame: Frame::new(name, formals),
        nesting_level: parent_level.nesting_level + 1
    }
}

use ExpInterm::*;
use Tree::Exp::*;
use Tree::Stm::*;

#[derive(Clone)]
pub struct Level {
    parent: Option<Frame>,
    pub frame: Frame,
    pub nesting_level: i64
}
pub type Access = frame::Access;
pub type Frag = frame::Frag;