extern crate uid;
pub mod escape;
pub mod frame;
#[allow(non_snake_case)]
#[macro_use]
pub mod Tree;
mod translate;
pub mod level;

use crate::ast::position::Pos;
#[allow(non_snake_case)]
pub use level::*;
pub type Access = frame::Access;
pub type Fragment = frame::Frag;

pub enum TransError {
    BreakError(Pos),
    DivByZero(Pos),
}

#[derive(Clone, Debug)]
pub enum EnvEntry {
    Var {
        access: Access,
        depth: i64,
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
pub fn initial_value_env(level: Level) -> ValueEnviroment {
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("flush"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("getchar"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("ord"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("chr"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("size"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("substring"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("concat"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("not"), Func {
        label: Label::new(),
        level: level.clone(),
        external: true
    });
    value_env.insert(Symbol::from("exit"), Func {
        label: Label::new(),
        level: level,
        external: true
    });
    value_env
}

static mut ACTUAL_LEVEL : i64 = 0;

impl Level {
    fn outermost() -> Level {
        Level {
            frame: Frame::new(
                Label::new(),
                vec![],
            ),
            nesting_depth: -1,
            id: LevelId::new(),
        }
    }

    fn new(parent_level: Level, name: Label, formals: Vec<bool>) -> Level {
        Level {
            frame: Frame::new(name, formals),
            nesting_depth: parent_level.nesting_depth + 1,
            id: LevelId::new(),
        }
    }

    fn alloc_arg(self: &mut Self, escape: bool) -> Access {
        self.frame.alloc_arg(escape)
    }

    fn alloc_local(self: &mut Self, escape: bool)  -> Access {
        self.frame.alloc_local(escape)
    }
}

use Tree::Exp::*;
use Tree::Stm::*;
use Tree::BinOp::*;
use Tree::RelOp::*;
use Tree::seq;

pub use translate::translate;