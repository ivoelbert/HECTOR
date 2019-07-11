use std::fs::{read_dir, read_to_string};

use super::super::ast::parser::{parse, ParseError};


#[test]
fn test_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).unwrap();
    for direntry in source_files {
        let path = direntry.unwrap().path();
        let contents = read_to_string(&path).unwrap();
        let parsed = parse(contents);
        match parsed {
            Ok(_) => assert!(true),
            Err(_) => panic!("{:?} deberia parsear bien pero falla", path)
        }
    }
}

#[test]
fn test_type() {
    let good_path = "./tiger_sources/type/";
    let source_files = read_dir(good_path).unwrap();
    for direntry in source_files {
        let path = direntry.unwrap().path();
        let contents = read_to_string(&path).unwrap();
        let parsed = parse(contents);
        match parsed {
            Ok(_) => assert!(true),
            Err(_) => panic!("{:?} deberia parsear bien (aunque despues no tipa) pero falla", path)
        }
    }
}

#[test]
fn test_syntax() {
    let syntax_path =  "./tiger_sources/syntax/";
    let source_files = read_dir(syntax_path).unwrap();
    for direntry in source_files {
        let path = direntry.unwrap().path();
        let contents = read_to_string(&path).unwrap();
        let parsed = parse(contents);
        match parsed {
            Err(_) => assert!(true),
            Ok(_) => panic!("{:?} debería fallar pero parsea bien", path)
        }
    }
}

