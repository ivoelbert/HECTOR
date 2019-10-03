use std::fs::{read_dir, read_to_string};

use super::super::ast::parser::{parse};
use super::super::ast::position::Pos;
use super::super::ast::*;

#[test]
fn test_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let parsed = parse(contents);
        match parsed {
            Ok(..) => (),
            Err(..) => panic!("{:?} deberia parsear bien pero falla", path),
        }
    }
}

#[test]
fn test_type() {
    let type_path = "./tiger_sources/type/";
    let source_files = read_dir(type_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let parsed = parse(contents);
        match parsed {
            Ok(..) => (),
            Err(..) => panic!(
                "{:?} deberia parsear bien (aunque despues no tipa) pero falla",
                path
            ),
        }
    }
}

#[test]
fn test_syntax() {
    let syntax_path = "./tiger_sources/syntax/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let parsed = parse(contents);
        match parsed {
            Err(..) => (),
            Ok(..) => panic!("{:?} deberia fallar pero parsea bien", path),
        }
    }
}

#[test]
fn test_parse_number() {
    let input = String::from("0");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Int(0),
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_string() {
    let input = String::from("\"perro\"");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::String(_),
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_breakexp() {
    let input = String::from("break");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Break,
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simplevar() {
    let input = String::from("foo");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Var(Var::Simple(_)),
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_subscriptvar() {
    let input = String::from("foo[0]");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Var(Var::Subscript(_, _)),
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_fieldvar() {
    let input = String::from("foo.baz");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Var(Var::Field(_, _)),
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_callexp() {
    let input = String::from("foo(1, \"perro\", baz)");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Call {..},
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_custom() {
    let input = String::from("(2 * 2 = 2 + 2) & 2 - 2 <> 2 + 2");
    let parsed = parse(input);

    match parsed {
        Ok(exp) => println!("Parsed expresion:\n\n{:?}\n\n", exp),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simple_sum() {
    let input = String::from("2 + 2");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Op {
                    oper: Oper::PlusOp,
                    ..
                },
            pos: Pos { line: 0, column: 0 },
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simple_mult() {
    let input = String::from("2 * 2");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Op {
                    oper: Oper::TimesOp,
                    ..
                },
            pos: Pos { line: 0, column: 0 },
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simple_comp() {
    let input = String::from("2 >= 2");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Op {
                    oper: Oper::GeOp,
                    ..
                },
            pos: Pos { line: 0, column: 0 },
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_recordexp() {
    let input = String::from("{first_name: \"Jhon\", last_name: \"doe\", age: 42}");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Record {..},
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_seqexp() {
    let input = String::from("(1;2)");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::Seq(_),
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_assignexp() {
    let input = String::from("foo = 42");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Assign {
                    var: Var::Simple(_),
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_ifexp() {
    let input = String::from("if 1 then 2 else 3");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::If {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_whileexp() {
    let input = String::from("for i :=0 to 100 do 1");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: _Exp::While {..},
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_forexp() {
    let input = String::from("for i :=0 to 100 do 1");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::For {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_letexp_functiondec() {
    // Este test se podria mejorar checkeando mas en profundidad el ast.
    let input = String::from("let function foo() = 1 in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}


#[test]
fn test_parse_letexp_typedec_namety() {
    // Este test se podria mejorar checkeando mas en profundidad el ast.
    let input = String::from("let type numeritos = int in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_letexp_typedec_recordty() {
    // Este test se podria mejorar checkeando mas en profundidad el ast.
    let input = String::from("let type name = {first_name: string, last_name: string} in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_letexp_typedec_arrayty() {
    // Este test se podria mejorar checkeando mas en profundidad el ast.
    let input = String::from("let type intArray = array of int in 2 ");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Let {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}


#[test]
fn test_parse_letexp_arrayexp() {
    // Este test se podria mejorar checkeando mas en profundidad el ast.
    let input = String::from("arrtype [10] of 0");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                _Exp::Array {
                    ..
                },
            ..
        }) => (),
        Ok(..) => panic!("mal parseado"),
        Err(..) => panic!("el parser falla en una expresion bien formada"),
    }
}
