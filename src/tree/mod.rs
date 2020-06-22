pub mod escape;
pub mod frame;
#[allow(non_snake_case)]
#[macro_use]
pub mod Tree;
pub mod translate;
pub mod level;

use crate::ast::position::Pos;
use serde::{Serialize};
use level::*;
type Access = frame::Access;
pub use level::Fragment;

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
        depth: i32
    }
}

use std::collections::HashMap;
use crate::ast::Symbol;
type ValueEnviroment = HashMap<Symbol, EnvEntry>;

pub fn initial_value_env() -> ValueEnviroment {
    use crate::externals::*;
    let externals = EXTERNALS
        .iter()
        .map(|External {name, is_runtime, ..}|
            if *is_runtime {
                let mut s = "+".to_string();
                s.push_str(*name);
                s
            } else {
                (*name).to_string()
            })
        .collect::<Vec<String>>();
    externals.iter().map(|name| -> (String, EnvEntry) {
        (name.to_string(), EnvEntry::Func {
            label: named_label(name),
            external: true,
            depth: 0
        })
    }).collect()
}

use Tree::Exp::*;
use Tree::Stm::*;
use Tree::BinOp::*;
use Tree::seq;

pub use translate::translate;