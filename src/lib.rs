#![deny(missing_docs)]
#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
)]
#![allow(
   clippy::implicit_return, // se contradice con otro?
   clippy::use_debug, // para debuguear el parser
   clippy::print_stdout,
   clippy::option_expect_used,
   clippy::wildcard_imports,
   clippy::enum_glob_use
)]

//! # HECTOR
//!
//! Heuristically Excessive Compiler for Tiger On Rust
//!
//! By Federico Badaloni & Ivo Elbert, Licenciatura en Ciencias de la Computación, FCEIA, UNR.

#[macro_use]
mod utils;
mod ast;
mod typecheck;
mod tree;
mod canonization;
mod emitter;

use typecheck::typecheck;
use utils::{set_panic_hook};

use wasm_bindgen::prelude::*;
extern crate serde_derive;
use serde::{Serialize};
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate lazy_static;

#[derive(Serialize, Debug)]
/// The final result of the compiler
pub struct CompilerResult {
    /// The parsed abstract syntax tree or a parsing error
    pub parse: Result<ast::AST, ast::parser::ParseError>,
    /// The typechecked abstract syntax tree or a typechecking error
    /// None if parsing failed
    pub typecheck: Option<Result<ast::AST, typecheck::TypeError>>,
    /// Typechecked abstract syntax tree that also has correct escape flags
    /// None if a previous step failed
    pub escape: Option<ast::AST>,
    /// Intermediate representation of the code
    /// None if a previous step failed
    pub translate: Option<Result<Vec<tree::Fragment>, tree::TransError>>,
    /// Semi-canonized intermediate representation or a translation error
    /// None if a previous step failed
    pub canon: Option<Vec<canonization::CanonFrag>>,
    /// The WebAssembly Text Format for the compiled code
    /// None if a previous step failed
    pub wasm: Option<String>,
    /// The resulting WebAssembly Binary
    /// None if a previous step failed
    pub bin: Option<Vec<u8>>
}

#[must_use]
/// Run all compiler stages and return the results for all of them
/// Mainly for examples and tests
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
    let (wasm_result, bin_result) = if let Some(canon) = canon_result.clone() {
        let (wasm, bin) = emitter::emit_module(canon);
        (Some(wasm), Some(bin))
    } else {(None, None)};
    CompilerResult{
        parse: parse_result,
        typecheck: typecheck_result,
        escape: escape_result,
        translate: translate_result,
        canon: canon_result,
        wasm: wasm_result,
        bin: bin_result
    }
}

#[wasm_bindgen]
#[must_use]
/// Returns a JSON encoded CompilerResult so we can load it in the browser
pub fn compile(source_code: &str) -> JsValue {
    set_panic_hook();
    console_log!("Running WASM!");

    if source_code == "" {
        console_log!("No code to compile, bye bye!");
        return JsValue::from(-1)
    }

    JsValue::from_serde(&run_compile(source_code)).expect("Should encode correctly")
}