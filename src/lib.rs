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
#![feature(start)]
extern crate lalrpop_util;
extern crate pathfinding;
extern crate snowflake;
extern crate typescript_definitions;

use typescript_definitions::TypeScriptify;
use crate::typescript_definitions::TypeScriptifyTrait;

#[macro_use]
mod utils;
mod ast;
mod typecheck;
mod tree;
#[allow(unused_imports)]
#[cfg(test)]
mod test;

use typecheck::{initial_type_env, initial_value_env, type_exp};
pub use utils::{log, set_panic_hook};

use wasm_bindgen::prelude::*;
extern crate serde_derive;
use serde::{Serialize};
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, TypeScriptify)]
pub struct CompilerResult {
    parse: Result<ast::AST, ast::parser::ParseError>,
    typecheck: Option<Result<ast::AST, typecheck::TypeError>>,
    escape: Option<ast::AST>,
    translate: Option<Result<Vec<tree::frame::Frag>, tree::TransError>>,
    canon: Option<Vec<tree::frame::Frag>>,
    wasm: Option<Vec<tree::frame::Frag>>
}

#[wasm_bindgen]
pub fn compile(source_code: &str) -> JsValue {
    set_panic_hook();
    console_log!("Running WASM!");

    if source_code == "" {
        console_log!("No code to compile, bye bye!");
        return JsValue::from(-1)
    }

    let parse_result = ast::parser::parse(source_code);
    let typecheck_result = if let Ok(ast) = &parse_result {
        Some(type_exp(ast.clone(), &initial_type_env(), &initial_value_env()))
    } else {None};
    let escape_result = if let Some(Ok(ast)) = &typecheck_result {
        Some(tree::escape::find_escapes(ast.clone()))
    } else {None};
    let translate_result = if let Some(ast) = &escape_result {
        Some(tree::translate(ast.clone()))
    } else {None};

    JsValue::from_serde(&CompilerResult{
        parse: parse_result,
        typecheck: typecheck_result,
        escape: escape_result,
        translate: translate_result,
        canon: None,
        wasm: None
    }).unwrap()
}

use tree::level::{Label, Temp};

#[test]
fn log_types() {
    println!("{}", CompilerResult::type_script_ify());
    println!("{}", tree::frame::Frag::type_script_ify());
    println!("{}", tree::Tree::Stm::type_script_ify());
    println!("{}", tree::frame::Frame::type_script_ify());
    println!("{}", tree::Tree::Exp::type_script_ify());
    println!("{}", tree::Tree::BinOp::type_script_ify());
    println!("{}", Temp::type_script_ify());
    // prints "export type MyStruct = { v: number };"
}