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
#[macro_use]
extern crate lalrpop_util;
extern crate pathfinding;

use wasm_bindgen::prelude::*;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod utils;
mod ast;
mod typecheck;
mod tree;

use ast::*;
use typecheck::{initial_type_env, initial_value_env, type_exp};


#[wasm_bindgen]
pub fn main(source_code: &str) -> JsValue {
    let ast = match ast::parser::parse(source_code) {
        Ok(ast) => ast,
        Err(parse_error) => {
            panic!()
        }
    };
    let typed_ast = match type_exp(ast, &initial_type_env(), &initial_value_env()) {
        Ok(ast) => ast,
        Err(typecheck_error) => {
            panic!()
        }
    };
    let escaped_ast = tree::escape::find_escapes(typed_ast);
    let tree_frags = match tree::translate(escaped_ast.clone()) {
        Ok(ast) => ast,
        Err(parse_error) => {
            panic!()
        }
    };
    JsValue::from_serde(&(escaped_ast, tree_frags)).unwrap()
}