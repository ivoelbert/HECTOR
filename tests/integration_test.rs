extern crate wasm_bindgen_test;
extern crate hector;
use std::fs::{read_dir, read_to_string};

#[test]
fn good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        println!("NOW COMPILING: {:?}", &path);
        let contents = read_to_string(&path).expect("read_to_string");
        let compile_result = hector::run_compile(&contents);
        match compile_result {
            hector::CompilerResult {
                parse: Ok(..),
                typecheck: Some(Ok(..)),
                translate: Some(Ok(..)),
                canon: Some(..),
                ..
            } => (),
            _ => panic!("COMPILING {:?} FAILS", &path)
        }
    }
}


#[test]
fn bad_syntax() {
    let syntax_path = "./tiger_sources/syntax/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let compile_result = hector::run_compile(&contents);
        match compile_result {
            hector::CompilerResult {
                parse: Err(..),
                typecheck: None,
                translate: None,
                canon: None,
                ..
            } => (),
            _ => panic!("COMPILING {:?} FAILS", &path)
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
        let compile_result = hector::run_compile(&contents);
        match compile_result {
            hector::CompilerResult {
                parse: Ok(..),
                typecheck: Some(Err(..)),
                translate: None,
                canon: None,
                ..
            } => (),
            _ => panic!("COMPILING {:?} FAILS", &path)
        }
    }
}

#[test]
fn bad_translate() {
    let syntax_path = "./tiger_sources/bad_translate/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let compile_result = hector::run_compile(&contents);
        match compile_result {
            hector::CompilerResult {
                parse: Ok(..),
                typecheck: Some(Ok(..)),
                translate: Some(Err(..)),
                canon: None,
                ..
            } => (),
            _ => panic!("COMPILING {:?} FAILS", &path)
        }
    }
}