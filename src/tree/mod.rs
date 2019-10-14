
pub mod escape;
pub mod frame;
#[allow(non_snake_case)]
pub mod Tree;
mod translate;
pub mod temp;

use crate::ast::position::Pos;
pub use temp::*;
pub use frame::{Frame};

pub enum TransError {
    BreakError(Pos),
    DivByZero(Pos),
}

#[derive(Clone, Debug)]
pub enum EnvEntry {
    Var {
        access: Access,
        level: i64,
    },
    Func {
        label: Label,
        level: Level,
        external: bool
    }
}

use std::collections::HashMap;
use crate::ast::Symbol;
pub type ValueEnviroment = HashMap<Symbol, EnvEntry>;


// revisar valores de retorno de estas
pub fn initial_value_env() -> ValueEnviroment {
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        label: Label::from("print"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("flush"), Func {
        label: Label::from("flush"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("getchar"), Func {
        label: Label::from("getchar"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("ord"), Func {
        label: Label::from("ord"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("chr"), Func {
        label: Label::from("chr"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("size"), Func {
        label: Label::from("size"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("substring"), Func {
        label: Label::from("substring"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("concat"), Func {
        label: Label::from("concat"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("not"), Func {
        label: Label::from("not"),
        level: outermost(),
        external: true
    });
    value_env.insert(Symbol::from("exit"), Func {
        label: Label::from("exit"),
        level: outermost(),
        external: true
    });
    value_env
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

use Tree::Exp::*;
use Tree::Stm::*;
use Tree::seq;

#[derive(Clone, Debug)]
pub struct Level {
    parent: Option<Frame>,
    pub frame: Frame,
    pub nesting_level: i64
}
pub type Access = frame::Access;
pub type Fragment = frame::Frag;

pub use translate::tranlate;