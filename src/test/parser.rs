use std::fs::{read_dir, read_to_string};

use crate::ast::parser::{parse};
use crate::ast::position::Pos;
use crate::ast::*;
use crate::typecheck::{TigerType};

/*
*   Naaaa naaa na na na na naaaaa, test good
*/
#[test]
fn good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let string_path = path.into_os_string().into_string().unwrap();
        println!("{:?}", string_path);
        let res = parse(contents.clone());
        match res {
            Ok(..) => (),
            Err(error) => panic!("Source {:?}\n Error: {:?}", string_path, error),
        }
    }
}

#[test]
fn bad_type() {
    let type_path = "./tiger_sources/type/";
    let source_files = read_dir(type_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let res = parse(contents.clone());
        match res {
            Err(..) => (),
            Ok(ast) => panic!("Source: {:?}\n AST: {:?}", contents, ast),
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
        let parsed = parse(contents);
        match parsed {
            Err(..) => (),
            Ok(..) => panic!("{:?} should fail, but parses ok", path),
        }
    }
}

#[test]
fn number() {
    let input = String::from("0");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Int(0),
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn string() {
    let input = String::from("\"perro\"");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::String(_),
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn breakexp() {
    let input = String::from("break");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Break,
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn simplevar() {
    let input = String::from("foo");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Var(Var{kind: VarKind::Simple(_), ..}),
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn subscriptvar() {
    let input = String::from("foo[0]");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Var(Var{kind: VarKind::Subscript(..), ..}),
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn fieldvar() {
    let input = String::from("foo.baz");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Var(Var{kind: VarKind::Field(..), ..}),
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn callexp() {
    let input = String::from("foo(1, \"perro\", baz)");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Call {..},
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well formed expression"),
    }
}

#[test]
fn custom() {
    let input = String::from("(2 * 2 = 2 + 2) & 2 - 2 <> 2 + 2");
    let parsed = parse(input);

    match parsed {
        Ok(exp) => println!("Parsed expresion:\n\n{:?}\n\n", exp),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn simple_sum() {
    let input = String::from("2 + 2");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Op {
                    oper: Oper::PlusOp,
                    ..
                },
            pos: Pos { line: 0, column: 0 },
            typ: _,
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn simple_mult() {
    let input = String::from("2 * 2");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Op {
                    oper: Oper::TimesOp,
                    ..
                },
            pos: Pos { line: 0, column: 0 },
            typ: _,
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn simple_comp() {
    let input = String::from("2 >= 2");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Op {
                    oper: Oper::GeOp,
                    ..
                },
            pos: Pos { line: 0, column: 0 },
            typ: _,
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn recordexp() {
    let input = String::from("{first_name: \"Jhon\", last_name: \"doe\", age: 42}");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Record {..},
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn seqexp() {
    let input = String::from("(1;2)");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::Seq(_),
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn assignexp() {
    let input = String::from("foo = 42");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Assign {
                    var: Var{kind: VarKind::Simple(_), ..},
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn ifexp() {
    let input = String::from("if 1 then 2 else 3");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::If {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn whileexp() {
    let input = String::from("for i :=0 to 100 do 1");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node: Exp::While {..},
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn forexp() {
    let input = String::from("for i :=0 to 100 do 1");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::For {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn letexp_functiondec() {
    let input = String::from("let function foo() = 1 in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}


#[test]
fn letexp_typedec_namety() {
    let input = String::from("let type numeritos = int in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn letexp_typedec_recordty() {
    let input = String::from("let type name = {first_name: string, last_name: string} in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}

#[test]
fn letexp_typedec_arrayty() {
    let input = String::from("let type intArray = array of int in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}


#[test]
fn letexp_arrayexp() {
    let input = String::from("arrtype [10] of 0");
    let parsed = parse(input);
    match parsed {
        Ok(AST {
            node:
                Exp::Array {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("wrong parsing"),
        Err(..) => panic!("parser fails in a well-formed expression"),
    }
}
