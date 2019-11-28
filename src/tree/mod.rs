extern crate uid;
pub mod escape;
pub mod frame;
#[allow(non_snake_case)]
#[macro_use]
pub mod Tree;
mod translate;
mod level;

use crate::ast::position::Pos;
#[allow(non_snake_case)]
use level::*;
type Access = frame::Access;
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
        external: bool,
    }
}

use std::collections::HashMap;
use crate::ast::Symbol;
type ValueEnviroment = HashMap<Symbol, EnvEntry>;


// TODO: check for correctnes
fn initial_value_env() -> ValueEnviroment {
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("flush"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("getchar"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("ord"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("chr"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("size"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("substring"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("concat"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("not"), Func {
        label: Label::new(),
        external: true,
    });
    value_env.insert(Symbol::from("exit"), Func {
        label: Label::new(),
        external: true,
    });
    value_env
}

use Tree::Exp::*;
use Tree::Stm::*;
use Tree::BinOp::*;
use Tree::seq;

pub use translate::translate;