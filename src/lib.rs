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
mod test;

use ast::*;
use typecheck::{initial_type_env, initial_value_env, type_exp};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn main(source_code: &str) -> JsValue {
    if source_code == "" {
        console_log!("OH SHIT!");
        return JsValue::from(-1)
    }
    console_log!("source: {}", source_code);
    console_log!("Inicio");
    let ast = match ast::parser::parse(source_code) {
        Ok(ast) => ast,
        Err(parse_error) => {
            panic!()
        }
    };
    console_log!("Parse OK");
    let typed_ast = match type_exp(ast, &initial_type_env(), &initial_value_env()) {
        Ok(ast) => ast,
        Err(typecheck_error) => {
            panic!()
        }
    };
    console_log!("Typecheck OK");
    let escaped_ast = tree::escape::find_escapes(typed_ast);
    console_log!("Escape OK");
    // let tree_frags = match tree::translate(escaped_ast.clone()) {
    //     Ok(ast) => ast,
    //     Err(parse_error) => {
    //         panic!()
    //     }
    // };
    // console_log!("Translate OK");
    JsValue::from_serde(&escaped_ast).unwrap()
}