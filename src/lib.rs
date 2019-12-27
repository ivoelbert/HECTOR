#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
)]
#![allow(
   clippy::missing_docs_in_private_items, // esto es una verdadera paja
   clippy::implicit_return, // se contradice con otro?
   clippy::use_debug, // para debuguear el parser
   clippy::print_stdout,
   clippy::needless_pass_by_value, // para tener los translate muertos
   dead_code,
)]
#![feature(inner_deref)]
#![feature(try_trait)]
#![feature(bind_by_move_pattern_guards)]
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod utils;
mod ast;
mod typecheck;
mod tree;

use ast::*;
use typecheck::{initial_type_env, initial_value_env, type_exp, TigerType, R};
//extern crate lalrpop_util;
extern crate pathfinding;

#[macro_use]
extern crate serde_derive;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn main() -> String {
   let exp = make_ast(Exp::Let {
       decs: vec![
           Dec::TypeDec(vec![(
               _TypeDec::new(
                   Symbol::from("List"),
                   Ty::Record(vec![
                       Field {
                           name: Symbol::from("head"),
                           typ: Ty::Name(Symbol::from("int")),
                           escape: false,
                       },
                       Field {
                           name: Symbol::from("tail"),
                           typ: Ty::Name(Symbol::from("List")),
                           escape: false,
                       }
                   ])
               ),
               Pos{line: 0, column: 1}
           )]),
           Dec::VarDec(
               _VarDec::new(
                   Symbol::from("foo"),
                   Some(Symbol::from("List")),
                   boxed_ast(Exp::Record {
                       fields: vec![
                           (Symbol::from("head"), boxed_ast(Exp::Int(1))),
                           (Symbol::from("tail"), boxed_ast(Exp::Record {
                               fields: vec![
                                   (Symbol::from("head"), boxed_ast(Exp::Int(2))),
                                   (Symbol::from("tail"), boxed_ast(Exp::Record {
                                       fields: vec![
                                           (Symbol::from("head"), boxed_ast(Exp::Int(3))),
                                           (Symbol::from("tail"), boxed_ast(Exp::Record {
                                               fields: vec![
                                                   (Symbol::from("head"), boxed_ast(Exp::Int(4))),
                                                   (Symbol::from("tail"), boxed_ast(Exp::Nil))
                                               ],
                                               typ: Symbol::from("List"),
                                           }))
                                       ],
                                       typ: Symbol::from("List"),
                                   }))
                               ],
                               typ: Symbol::from("List"),
                           }))
                       ],
                       typ: Symbol::from("List"),
                   })
               ),
               Pos{line: 0, column: 2}
           )],
       body: boxed_ast(Exp::Var(
           make_var(VarKind::Field(
               boxed_var(VarKind::Simple(Symbol::from("foo"))),
               Symbol::from("head")
           ))
       ))
   });
   let type_env = initial_type_env();
   let value_env = initial_value_env();
   let res = type_exp(exp.clone(), &type_env, &value_env);

   match &res {
       Ok(ast) if *ast.typ == TigerType::TInt(R::RW) => serde_json::to_string(ast).unwrap(),
       Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
       Err(type_error) => panic!("type error: {:?}", type_error)
   }
}