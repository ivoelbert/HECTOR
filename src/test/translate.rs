use std::fs::{read_dir, read_to_string};
use std::sync::Arc;

use crate::ast::*;
use crate::ast::position::*;
use crate::ast::parser::parse;
use crate::tree::escape::find_escapes;
use crate::typecheck::*;
use crate::tree::translate;

#[test]
fn translate_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("parser error");
        let type_env = TypeEnviroment::new();
        let value_env = ValueEnviroment::new();
        let typed = type_exp(ast.clone() , &type_env, &value_env).unwrap();
        let escaped = find_escapes(typed);
        let translated = translate(escaped);
        match translated {
            Ok(..) => (),
            Err(type_error) => panic!("Source: {:?}\n, AST: {:?}, Type Error: {:?}", &path, &ast, &type_error)
        }
    }
}