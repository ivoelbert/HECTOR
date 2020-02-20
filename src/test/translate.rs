extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use std::fs::{read_dir, read_to_string};
use std::sync::Arc;

use crate::ast;
use crate::ast::AST;
use crate::ast::position::*;
use crate::ast::parser::parse;
use crate::tree::escape::find_escapes;
use crate::typecheck::{type_exp, TigerType};
use crate::tree::translate;
use crate::tree::*;
use crate::tree::level::*;
use Tree::Exp::*;
use Tree::Stm::*;
use Tree::BinOp::*;
use Tree::seq;

#[test]
#[wasm_bindgen_test]
fn translate_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("parser error");
        let type_env = crate::typecheck::initial_type_env();
        let value_env = crate::typecheck:: initial_value_env();
        let typed = type_exp(ast.clone() , &type_env, &value_env).unwrap();
        let escaped = find_escapes(typed);
        let translated = translate(escaped);
        match translated {
            Ok(..) => (),
            Err(type_error) => panic!("Source: {:?}\n, AST: {:?}, Type Error: {:?}", &path, &ast, &type_error)
        }
    }
}

#[test]
#[wasm_bindgen_test]
fn break_no_labels_error() {
    let exp = AST {
        node: ast::Exp::Break,
        pos: Pos { line: 0, column: 0 },
        typ: Arc::new(TigerType::TUnit)
    };
    let level = Level::outermost();
    let res = translate::breakexp::trans_stm(&exp, level, &initial_value_env(), &vec![], vec![]);
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
#[wasm_bindgen_test]
fn break_none_label_error() {
    let exp = AST {
        node: ast::Exp::Break,
        pos: Pos { line: 0, column: 0 },
        typ: Arc::new(TigerType::TUnit)
    };
    let level = Level::outermost();
    let res = translate::breakexp::trans_stm(&exp, level, &initial_value_env(), &vec![], vec![]);
    match res {
        Err(TransError::BreakError(_)) => (),
        Err(..) => panic!("wrong error"),
        Ok(..) => panic!("shouldn't translate"),
    }
}

#[test]
#[wasm_bindgen_test]
fn break_ok() {
    let exp = AST {
        node: ast::Exp::Break,
        pos: Pos { line: 0, column: 0 },
        typ: Arc::new(TigerType::TUnit)
    };
    let level = Level::outermost();
    let res = translate::breakexp::trans_stm(&exp, level, &initial_value_env(), &vec![Some(newlabel())], vec![]);
    match res {
        Ok((JUMP(NAME(_), _), _, fragments)) => {
            assert!(fragments.is_empty());
        }
        Ok(..) => panic!("wrong translation"),
        Err(..) => panic!("should translate"),
    }
}

#[test]
#[wasm_bindgen_test]
fn stringexp_ok() {
    let exp = AST {
        node: ast::Exp::String(String::from("lorem ipsum")),
        pos: Pos {
            line: 0,
            column: 0,
        },
        typ: Arc::new(TigerType::TString)
    };
    let level = Level::outermost();
    let value_env = initial_value_env();
    let res = translate::stringexp::trans_exp(&exp, level, &value_env, &vec![], vec![]);
    match res {
        Ok((NAME(_), _level, fragments)) => {
            assert!(!fragments.is_empty());
        },
        Ok(..) => panic!("wrong result"),
        Err(..) => panic!("should translate"),
    }
}

// TEST: let
//      declaring variable modifies the level
//      declaring a function inside a variable adds a fragment
//      declaring a type doesn't modify anything
//      declaring a variable inside a function doesn't modify the current level
//      can declare arrays
//      can declare records
//      a function body with a break inside a while fails