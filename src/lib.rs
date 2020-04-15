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
extern crate uuid;


#[macro_use]
mod utils;
mod ast;
mod typecheck;
mod tree;
mod canonization;

use typecheck::typecheck;
pub use utils::{log, set_panic_hook};

use wasm_bindgen::prelude::*;
extern crate serde_derive;
use serde::{Serialize};
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct CompilerResult {
    pub parse: Result<ast::AST, ast::parser::ParseError>,
    pub typecheck: Option<Result<ast::AST, typecheck::TypeError>>,
    pub escape: Option<ast::AST>,
    pub translate: Option<Result<Vec<tree::Fragment>, tree::TransError>>,
    pub canon: Option<Vec<canonization::CanonFrag>>,
    pub wasm: Option<Vec<tree::Fragment>>
}

pub fn run_compile(source_code: &str) -> CompilerResult {
    let parse_result = ast::parser::parse(source_code);
    let typecheck_result = if let Ok(ast) = &parse_result {
        Some(typecheck(ast.clone()))
    } else {None};
    let escape_result = if let Some(Ok(ast)) = &typecheck_result {
        Some(tree::escape::find_escapes(ast.clone()))
    } else {None};
    let translate_result = if let Some(ast) = &escape_result {
        Some(tree::translate(ast.clone()))
    } else {None};
    let canon_result = if let Some(Ok(frags)) = &translate_result {
        Some(canonization::canonize(frags.clone()))
    } else {None};
    CompilerResult{
        parse: parse_result,
        typecheck: typecheck_result,
        escape: escape_result,
        translate: translate_result,
        canon: canon_result,
        wasm: None
    }
}

#[wasm_bindgen]
pub fn compile(source_code: &str) -> JsValue {
    set_panic_hook();
    console_log!("Running WASM!");

    if source_code == "" {
        console_log!("No code to compile, bye bye!");
        return JsValue::from(-1)
    }

    JsValue::from_serde(&run_compile(source_code)).unwrap()
}