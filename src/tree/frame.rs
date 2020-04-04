// Arquitecture-specific details should be abstracted here.
// To make it easier to target diffent arquitectures, we could make Frame into a trait.

use super::{Label, GlobalTemp, level::*, Tree};
use Tree::{Exp::*, Stm::*};
use serde::{Serialize};
use uuid::Uuid;

pub static WORD_SIZE: i32 = 4;
pub static FRAME_POINTER : &str = "fp";
pub static STACK_POINTER : &str = "sp";
pub static RETURN_VALUE : &str = "rv";


#[derive(Clone, Debug, Serialize)]
pub struct Frame {
    name: String,
    label: Label,
    formals: Vec<(String, bool)>,
    locals: Vec<LocalTemp>,
}

pub type MemAddress = i32;
pub type LocalTemp = String;

#[derive(Clone, Debug, Serialize)]
pub enum Access {
    InLocal(LocalTemp),
    InGlobal(GlobalTemp),
    InMem(MemAddress)
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
    pub fn new(name: String, label: Label) -> Self {
        Frame {
            name,
            label,
            formals: vec![],
            locals: vec![],
        }
    }

    pub fn alloc_arg(self: &mut Self, name: String, escape: bool) -> Access {
        let label = named_local(&name);
        self.formals.push((label.clone(), escape));
        match escape {
            true => Access::InGlobal(label),
            false => Access::InLocal(label)
        }
    }

    pub fn alloc_local(self: &mut Self, escape: bool, name: Option<String>) -> Access {
        let label = if let Some(name) = name {named_local(&name)} else {newlocal()};
        self.locals.push(label.clone());
        match escape {
            true => Access::InGlobal(label),
            false => Access::InLocal(label)
        }
    }

    pub fn generate_move_escaped_arguments_statement(self: &Self) -> Vec<Tree::Stm>{
        // a.k.a. procEntryExit1

        self.formals.iter().map(|(name, escape)| -> Tree::Stm {
            if *escape {
                MOVE(Box::new(GLOBAL(name.clone())), Box::new(LOCAL(name.clone())))
            } else {
                EXP(Box::new(CONST(0)))
            }
        }).collect()
    }

    pub fn formals(self: &Self) -> Vec<Access> {
        // Genera los access segun:
        // - El diseño de frame que elegimos
        // - La convencion de llamada (todo lo que se puede en locals)
        // - El vector de escapes.
        // Tiene que ser consistente con como incrementamos el contador de locals en el constructor.
        use Access::*;
        self.formals.iter().map(|(name, escape)| -> Access {
            if *escape {
                InGlobal(name.clone())
            } else {
                InLocal(name.clone())
            }
        }).collect()
    }
}

pub fn external_call(proc_name: String, proc_label: Label, args: Vec<Tree::Exp>) -> Tree::Exp {
    // TODO: ajustar segun convencion de llamada de lo que sea que usemos para el runtime
    CALL(proc_name, Box::new(NAME(proc_label)), args)
}

pub fn newlocal() -> GlobalTemp {
    Uuid::new_v4().to_string()
}

pub fn named_local(name: &str) -> GlobalTemp {
    vec![name.to_string(), "_".to_string(), Uuid::new_v4().to_string()].concat()
}