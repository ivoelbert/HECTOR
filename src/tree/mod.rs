pub mod escape;
pub mod frame;
#[allow(non_snake_case)]
#[macro_use]
pub mod Tree;
pub mod translate;
pub mod level;

use crate::ast::position::Pos;
use serde::{Serialize};
#[allow(non_snake_case)]
use level::*;
type Access = frame::Access;
pub type Fragment = frame::Frag;

#[derive(Debug, Serialize)]
pub enum TransError {
    BreakError(Pos),
    DivByZero(Pos),
}

#[derive(Clone, Debug)]
pub enum EnvEntry {
    Var {
        access: Access,
        depth: i32,
    },
    Func {
        label: Label,
        external: bool,
    }
}

use std::collections::HashMap;
use crate::ast::Symbol;
type ValueEnviroment = HashMap<Symbol, EnvEntry>;

pub fn initial_value_env() -> ValueEnviroment {
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("flush"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("getchar"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("ord"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("chr"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("size"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("substring"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("concat"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("not"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("exit"), Func {
        label: newlabel(),
        external: true,
    });
    // Runtime functions are preceded by a + to avoid collision with user-defined functions/variables.
    value_env.insert(Symbol::from("+alloc_array"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+alloc_record"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+str_equals"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+str_not_equals"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+str_less"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+str_less_or_equals"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+str_greater"), Func {
        label: newlabel(),
        external: true,
    });
    value_env.insert(Symbol::from("+str_greater_or_equals"), Func {
        label: newlabel(),
        external: true,
    });
    value_env
}

use Tree::Exp::*;
use Tree::Stm::*;
use Tree::BinOp::*;
use Tree::seq;

pub use translate::translate;