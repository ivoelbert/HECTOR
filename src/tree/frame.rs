// Arquitecture-specific details should be abstracted here.
// To make it easier to target diffent arquitectures, we could make Frame into a trait.

use super::{Label, GlobalTemp, Tree, level::named_global};
use Tree::{Exp::*, Stm::*, BinOp::*};
use serde::{Serialize};
use uuid::Uuid;

pub static WORD_SIZE: i32 = 4;
pub static FRAME_POINTER : &str = "fp";
pub static STACK_POINTER : &str = "sp";
pub static RETURN_VALUE : &str = "rv";

pub static STATIC_LINK_OFFSET : i32 = 0;


#[derive(Clone, Debug, Serialize)]
pub struct Frame {
    pub label: Label,
    formals: Vec<(String, bool)>,
    // locals: Vec<LocalTemp>,
    memindex: i32,

}

pub type MemAddress = i32;
pub type LocalTemp = String;

#[derive(Clone, Debug, Serialize)]
pub enum Access {
    Local(LocalTemp),
    Global(GlobalTemp),
    Mem(MemAddress)
}

impl Frame {
    pub fn new(label: Label) -> Self {
        Frame {
            label,
            formals: vec![],
            memindex: -1
        }
    }

    pub fn alloc_arg(self: &mut Self, name: String, escape: bool) -> Access {
        self.formals.push((name.clone(), escape));
        if escape {
            self.memindex += 1;
            Access::Mem(self.memindex)
        } else { Access::Local(name) }
    }

    pub fn alloc_local(self: &mut Self, escape: bool, name: Option<String>) -> Access {
        let label = if let Some(name) = name {name} else {unique_named_local("-alloc-local")};
        if escape {
            self.memindex += 1;
            Access::Mem(self.memindex)
        } else { Access::Local(label) }
    }

    pub fn generate_move_escaped_arguments_statement(self: &Self) -> Tree::Stm {
        // a.k.a. procEntryExit1
        let moves = self.formals.iter().fold(
            (vec![], -1),
            |(mut stms, mut current_index): (Vec<Tree::Stm>, i32), (name, escape): &(String, bool)| -> (Vec<Tree::Stm>, i32) {
                if *escape {
                    current_index += 1;
                    stms.push(MOVE(
                        Box::new(MEM(Box::new(BINOP(PLUS,
                            Box::new(GLOBAL(named_global(FRAME_POINTER))),
                            Box::new(CONST(current_index * WORD_SIZE) )
                        )))),
                        Box::new(LOCAL(name.clone()))))
                };
                (stms, current_index)
            }
        ).0;
        Tree::seq(moves)
    }

    pub fn formals(&self) -> Vec<(String, Access)> {
        // Genera los access segun:
        // - El diseño de frame que elegimos
        // - La convencion de llamada (todo lo que se puede en locals)
        // - El vector de escapes.
        // Tiene que ser consistente con como incrementamos el contador de locals en el constructor.
        self.formals.iter().fold(
            (vec![], -1),
            |(mut formals, mut current_index): (Vec<(String, Access)>, i32), (name, escape): &(String, bool)| -> (Vec<(String, Access)>, i32) {
                formals.push(if *escape {
                    current_index += 1;
                    (name.to_string(), Access::Mem(current_index))
                } else {
                    (name.to_string(), Access::Local(name.clone()))
                });
                (formals, current_index)
        }).0
    }

    pub fn static_size(&self) -> i32 {
        (self.memindex + 1) * WORD_SIZE
    }
}

pub fn external_call(proc_label: Label, args: Vec<Tree::Exp>) -> Tree::Exp {
    // TODO: ajustar segun convencion de llamada de lo que sea que usemos para el runtime
    CALL(Box::new(NAME(proc_label)), args)
}

// pub fn newlocal() -> GlobalTemp {
//     Uuid::new_v4().to_string()
// }

// pub fn named_local(name: &str) -> GlobalTemp {
//     name.to_string()
// }

pub fn unique_named_local(name: &str) -> GlobalTemp {
    vec![name.to_string(), "_".to_string(), Uuid::new_v4().to_string()].concat()
}