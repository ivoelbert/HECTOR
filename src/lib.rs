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
   clippy::missing_inline_in_public_items,
   dead_code,
)]
#![feature(inner_deref)]
#![feature(try_trait)]
#![feature(bind_by_move_pattern_guards)]
extern crate lalrpop_util;
extern crate pathfinding;
extern crate snowflake;


use wasm_bindgen::prelude::*;
extern crate serde_derive;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
mod utils;
mod ast;
mod typecheck;
mod tree;
#[allow(unused_imports)]
mod test;

use typecheck::{initial_type_env, initial_value_env, type_exp};
use utils::{log, set_panic_hook};

#[wasm_bindgen]
pub fn main(source_code: &str) -> JsValue {
    set_panic_hook();
    console_log!("Running WASM!");

    if source_code == "" {
        console_log!("No code to compile, bye bye!");
        return JsValue::from(-1)
    }

    JsValue::from("Chorizo")

    /*
    if source_code == "" {
        console_log!("OH SHIT!");
        return JsValue::from(-1)
    }
    console_log!("source: {}", source_code);
    console_log!("Inicio");
    let ast = match ast::parser::parse(source_code) {
        Ok(ast) => ast,
        Err(parse_error) => {
            console_log!("Parse Error: {:?}", parse_error);
            return JsValue::from_serde(&parse_error).unwrap()
        }
    };
    console_log!("Parse OK");
    let typed_ast = match type_exp(ast, &initial_type_env(), &initial_value_env()) {
        Ok(ast) => ast,
        Err(typecheck_error) => {
            console_log!("Typechecking Error: {:?}", typecheck_error);
            return JsValue::from_serde(&typecheck_error).unwrap()
        }
    };
    console_log!("Typecheck OK");
    let escaped_ast = tree::escape::find_escapes(typed_ast);
    console_log!("Escape OK");
    let tree_frags = match tree::translate(escaped_ast.clone()) {
        Ok(interm_exp) => interm_exp,
        Err(trans_error) => {
            console_log!("Translation Error: {:?}", trans_error);
            return JsValue::from_serde(&trans_error).unwrap()
        }
    };
    console_log!("Translate OK");
    JsValue::from_serde(&tree_frags).unwrap()
}