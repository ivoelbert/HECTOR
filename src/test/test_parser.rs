// use std::fs::{read_dir, read_to_string};

// use ast::parser::{parse, ParseError};


// #[test]
// fn test_good() {
//     let good_path = "./tiger_sources/good/";
//     let source_files = read_dir(good_path).unwrap();
//     for direntry in source_files {
//         let path = direntry.unwrap().path();
//         let mut contents = read_to_string(&path).unwrap();
//         parse(contents).expect("Compilation filed");
//     }
// }

// #[test]
// fn test_syntax() {
//     let syntax_path =  "./tiger_sources/syntax/";
//     let source_files = read_dir(syntax_path).unwrap();
//     for direntry in source_files {
//         let path = direntry.unwrap().path();
//         let mut contents = read_to_string(&path).unwrap();
//         assert_eq!(parse(contents), Err);
//     }
// }