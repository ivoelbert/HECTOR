use std::fs::{read_dir, read_to_string};
use std::sync::Arc;

use crate::ast;
use crate::ast::parser::parse;
use crate::tree::escape::find_escapes;
use crate::typecheck::typecheck;
use crate::tree::translate;
use crate::canonization;

use crate::utils::log;

/*
*   Naaaa naaa na na na na naaaaa, test good
*/
#[test]
fn good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        println!("NOW CANONIZING: {:?}", &path);
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("parser error");
        let typed = typecheck(ast.clone()).unwrap();
        let escaped = find_escapes(typed);
        let translated = translate(escaped).unwrap();
        let _ = canonization::canonize(translated);
    }
}

#[test]
fn parser_bad_type() {
    let type_path = "./tiger_sources/type/";
    let source_files = read_dir(type_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let string_path = path.into_os_string().into_string().unwrap();
        let res = parse(&contents.clone());
        match res {
            Ok(..) => (),
            Err(error) => panic!("Source {:?}\n Error: {:?}", string_path, error),
        }
    }
}

#[test]
fn parser_bad_syntax() {
    let syntax_path = "./tiger_sources/syntax/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let parsed = parse(&contents);
        match parsed {
            Err(..) => (),
            Ok(..) => panic!("{:?} should fail, but parses ok", path),
        }
    }
}

#[test]
fn bad_type() {
    let syntax_path = "./tiger_sources/type/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("falla el parser");
        let typed = typecheck(ast.clone());
        match typed {
            Err(..) => (),
            Ok(res) => panic!("Source: {:?}, AST: {:?}\n Type: {:?}", &path, ast, res),
        }
    }
}