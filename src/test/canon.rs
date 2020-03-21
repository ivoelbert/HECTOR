use std::fs::{read_dir, read_to_string};
use std::sync::Arc;

use crate::ast;
use crate::ast::parser::parse;
use crate::tree::escape::find_escapes;
use crate::typecheck::{type_exp, TigerType};
use crate::tree::translate;
use crate::canonization;

use crate::utils::log;

#[test]
fn canon_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        println!("NOW CANONIZING: {:?}", &path);
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("parser error");
        let type_env = crate::typecheck::initial_type_env();
        let value_env = crate::typecheck:: initial_value_env();
        let typed = type_exp(ast.clone() , &type_env, &value_env).unwrap();
        let escaped = find_escapes(typed);
        let translated = translate(escaped).unwrap();
        let _ = canonization::canonize(translated);
    }
}