use std::fs::{read_dir, read_to_string};

use super::super::ast::parser::{parse, ParseError};
use super::super::ast::position::Pos;
use super::super::ast::tigerabs::_Exp::*;
use super::super::ast::tigerabs::*;

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
            Err(_) => panic!("{:?} deberia parsear bien pero falla", path),
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
            Err(_) => panic!(
                "{:?} deberia parsear bien (aunque despues no tipa) pero falla",
                path
            ),
        }
    }
}

#[test]
fn test_syntax() {
    let syntax_path = "./tiger_sources/syntax/";
    let source_files = read_dir(syntax_path).unwrap();
    for direntry in source_files {
        let path = direntry.unwrap().path();
        let contents = read_to_string(&path).unwrap();
        let parsed = parse(contents);
        match parsed {
            Err(_) => assert!(true),
            Ok(_) => panic!("{:?} debería fallar pero parsea bien", path),
        }
    }
}

#[test]
fn test_parse_number() {
    let input = String::from("0");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: IntExp(0),
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_string() {
    let input = String::from("\"perro\"");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: StringExp(_),
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_breakexp() {
    let input = String::from("break");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: BreakExp,
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simplevar() {
    let input = String::from("foo");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: VarExp(Var::SimpleVar(_)),
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_subscriptvar() {
    let input = String::from("foo[0]");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: VarExp(Var::SubscriptVar(_, _)),
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_fieldvar() {
    let input = String::from("foo.baz");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: VarExp(Var::FieldVar(_, _)),
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_callexp() {
    let input = String::from("foo(1, \"perro\", baz)");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: CallExp { func: _, args: _ },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}


#[test]
fn test_parse_custom() {
    let input = String::from(r#"
        let
            var N := 8

            type intArray = array of int

            var row := intArray [ N ]
        in
            0
        end
    "#);
    let parsed = parse(input);

    match parsed {
        Ok(exp) => println!("Parsed expresion:\n\n{:?}\n\n", exp),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}


#[test]
fn test_parse_simple_sum() {
    let input = String::from("2 + 2");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                OpExp {
                    left: _,
                    oper: Oper::PlusOp,
                    right: _,
                },
            pos: Pos { line: 0, column: 0 },
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simple_mult() {
    let input = String::from("2 * 2");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                OpExp {
                    left: _,
                    oper: Oper::TimesOp,
                    right: _,
                },
            pos: Pos { line: 0, column: 0 },
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_simple_comp() {
    let input = String::from("2 >= 2");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                OpExp {
                    left: _,
                    oper: Oper::GeOp,
                    right: _,
                },
            pos: Pos { line: 0, column: 0 },
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_recordexp() {
    let input = String::from("{first_name: \"Jhon\", last_name: \"doe\", age: 42}");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: RecordExp { fields: _, typ: _ },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_seqexp() {
    let input = String::from("(1;2)");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: SeqExp(_),
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_assignexp() {
    let input = String::from("foo := 42");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                AssignExp {
                    var: Var::SimpleVar(_),
                    exp: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_ifexp() {
    let input = String::from("if 1 then 2 else 3");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                IfExp {
                    test: _,
                    then_: _,
                    else_: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_whileexp() {
    let input = String::from("while 1 do ()");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node: WhileExp { test: _, body: _ },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}

#[test]
fn test_parse_forexp() {
    let input = String::from("for i :=0 to 100 do 1");
    let parsed = parse(input);
    match parsed {
        Ok(Exp {
            node:
                ForExp {
                    var: _,
                    escape: _,
                    lo: _,
                    hi: _,
                    body: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
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
                LetExp {
                    decs: _,
                    body: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
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
                LetExp {
                    decs: _,
                    body: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
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
                LetExp {
                    decs: _,
                    body: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
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
                LetExp {
                    decs: _,
                    body: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
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
                ArrayExp {
                    typ: _,
                    size: _,
                    init: _,
                },
            pos: _,
        }) => assert!(true),
        Ok(_) => panic!("mal parseado"),
        Err(_) => panic!("el parser falla en una expresion bien formada"),
    }
}
