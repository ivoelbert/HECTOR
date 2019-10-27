use std::fs::{read_dir, read_to_string};

use crate::ast::*;
use crate::ast::position::*;
use super::super::ast::parser::parse;

use crate::typecheck::*;

#[test]
fn good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let exp = parse(contents).expect("parser error");
        let type_env = TypeEnviroment::new();
        let value_env = ValueEnviroment::new();
        let res = type_exp(&exp , &type_env, &value_env);
        match res {
            Ok(..) => (),
            Err(type_error) => panic!("Expresion: {:?}\n Type Error: {:?}", exp, type_error)
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
        let exp = parse(contents).expect("falla el parser");
        let type_env = TypeEnviroment::new();
        let value_env = ValueEnviroment::new();
        let res = type_exp(&exp , &type_env, &value_env);
        match res {
            Err(..) => (),
            Ok(tiger_type) => panic!("Expresion: {:?}\n Type: {:?}", exp, tiger_type),
        }
    }
}

fn possed_exp(exp: _Exp) -> Exp {
    Exp {node: exp, pos: Pos {line: 0, column: 0}}
}

#[test]
fn unitexp() {
    let exp = Exp {
        node: _Exp::Unit,
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn nilexp() {
    let exp = Exp {
        node: _Exp::Nil,
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TNil) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn breakexp() {
    let exp = Exp {node: _Exp::Break, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn intexp() {
    let exp = Exp {
        node: _Exp::Int(1),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn stringexp() {
    let exp = Exp {
        node: _Exp::String(String::from("lorem ipsum")),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TString) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn varexp_simplevar_ok() {
    let exp = Exp {
        node: _Exp::Var(Var::Simple(Symbol::from("foo"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{ty: TigerType::TInt(R::RW),});
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn varexp_simplevar_no_declarada() {
    let exp = Exp {
        node: _Exp::Var(Var::Simple(Symbol::from("foo"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn varexp_simplevar_no_es_simple() {
    let exp = Exp {
        node: _Exp::Var(Var::Simple(Symbol::from("f"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![],
        result: TigerType::TUnit,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn varexp_fieldvar_ok() {
    let exp = Exp {
        node: _Exp::Var(Var::Field(Box::new(Var::Simple(Symbol::from("foo"))),Symbol::from("bar"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = TigerType::TRecord(
            vec![(Box::new(String::from("bar")),
                Box::new(TigerType::TInt(R::RW)),
                0)], TypeId::new());
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn varexp_fieldvar_field_inexistente() {
    let exp = Exp {
        node: _Exp::Var(Var::Field(Box::new(Var::Simple(Symbol::from("foo"))),Symbol::from("perro"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = TigerType::TRecord(
            vec![(
                Box::new(String::from("bar")),
                Box::new(TigerType::TInt(R::RW)),
                0
            )],
            TypeId::new(),
        );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::FieldDoesNotExist(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn varexp_fieldvar_sobre_tipo_no_record() {
    let exp = Exp {
        node: _Exp::Var(Var::Field(Box::new(Var::Simple(Symbol::from("foo"))),Symbol::from("bar"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = TigerType::TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotRecordType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn varexp_subscriptvar_ok() {
    let exp = Exp {
        node: _Exp::Var(
            Var::Subscript(Box::new(Var::Simple(Symbol::from("foo"))),
            Box::new(Exp {
                node: _Exp::Int(0),
                pos: Pos {
                    line: 0,
                    column: 0,
            }}))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = TigerType::TArray(
        Box::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn varexp_subscriptvar_indice_no_entero() {
    let exp = Exp {
        node: _Exp::Var(
            Var::Subscript(Box::new(Var::Simple(Symbol::from("foo"))),
            Box::new(Exp {
                node: _Exp::String(String::from("una string de indice :o")),
                pos: Pos {
                    line: 0,
                    column: 0,
            }}))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = TigerType::TArray(
        Box::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::SunscriptNotInteger(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn varexp_subscriptvar_no_array() {
    let exp = Exp {
        node: _Exp::Var(
            Var::Subscript(Box::new(Var::Simple(Symbol::from("foo"))),
            Box::new(Exp {
                node: _Exp::Int(0),
                pos: Pos {
                    line: 0,
                    column: 0,
            }}))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = TigerType::TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotArrayType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn callexp_ok() {
    let exp = Exp {
        node: _Exp::Call {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![],
        result: TigerType::TUnit,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn callexp_args_de_mas() {
    let exp = Exp {
        node: _Exp::Call {
            func: Symbol::from("f"),
            args: vec![Exp {
                node: _Exp::Int(1),
                pos: Pos {line: 0, column: 0}
            }],
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![],
        result: TigerType::TUnit,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TooManyArguments(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn callexp_args_de_menos() {
    let exp = Exp {
        node: _Exp::Call {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![TigerType::TInt(R::RW)],
        result: TigerType::TUnit,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TooFewArguments(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn callexp_funcion_no_declarada() {
    let exp = Exp {
        node: _Exp::Call {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredFunction(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn opexp_ok() {
    let exp = Exp {
        node: _Exp::Op {
            left: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
            oper: Oper::PlusOp,
            right: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn opexp_tipos_distintos() {
    let exp = Exp {
        node: _Exp::Op {
            left: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
            oper: Oper::PlusOp,
            right: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn recordexp_ok() {
    let exp = Exp {
        node: _Exp::Record {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = TigerType::TRecord(
            vec![(Box::new(String::from("baz")),
                Box::new(TigerType::TInt(R::RW)),
                0)], TypeId::new());
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(return_type) => assert!(return_type == foo_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn recordexp_tipo_inexistente() {
    let exp = Exp {
        node: _Exp::Record {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn recordexp_con_tipo_no_record() {
    let exp = Exp {
        node: _Exp::Record {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    type_env.insert(Symbol::from("FooType"), TigerType::TInt(R::RW));
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotRecordType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn arrayexp_ok() {
    let exp = Exp {node: _Exp::Array {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::Int(2), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = TigerType::TArray(
        Box::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(return_type) => assert!(return_type == foo_type),
        Err(..) => panic!("array")
    }
}

#[test]
fn arrayexp_size_no_int() {
    let exp = Exp {node: _Exp::Array {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = TigerType::TArray(
        Box::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerSize(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn arrayexp_tipos_distintos() {
    let exp = Exp {node: _Exp::Array {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = TigerType::TArray(
        Box::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn arrayexp_tipo_no_array() {
    let exp = Exp {node: _Exp::Array {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = TigerType::TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotArrayType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn arrayexp_tipo_no_existe() {
    let exp = Exp {node: _Exp::Array {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn seqexp_ok() {
    let exp = Exp {
        node: _Exp::Seq(vec![
            Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}},
            Exp {node: _Exp::Int(2), pos: Pos {line: 0, column: 0}},
        ]),
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}
// Se puede testear algo mas de _Exp::Seq? Hay alguna condicion del ultimo tipo?

#[test]
fn assignexp_ok() {
    let exp = Exp {node: _Exp::Assign{
        var: Var::Simple(Symbol::from("foo")),
        exp: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: TigerType::TInt(R::RW),
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn assignexp_variable_no_existe() {
    let exp = Exp {node: _Exp::Assign{
        var: Var::Simple(Symbol::from("foo")),
        exp: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn assignexp_tipos_distintos() {
    let exp = Exp {node: _Exp::Assign{
        var: Var::Simple(Symbol::from("foo")),
        exp: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: TigerType::TInt(R::RW),
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn assignexp_variable_read_only() {
    let exp = Exp {node: _Exp::Assign{
        var: Var::Simple(Symbol::from("i")),
        exp: Box::new(Exp {node: _Exp::Int(2), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: TigerType::TInt(R::RO),
    };
    value_env.insert(Symbol::from("i"), env_entry);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::ReadOnlyAssignment(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn ifexp_ok() {
    let exp = Exp {node: _Exp::If {
        test: Box::new(Exp {node: _Exp::Int(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: _Exp::Int(2), pos: Pos {line: 0, column: 0}}))
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(_)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn ifexp_test_no_entero() {
    let exp = Exp {node: _Exp::If {
        test: Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: _Exp::Int(2), pos: Pos {line: 0, column: 0}}))
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerCondition(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn ifexp_tipos_then_else_distintos() {
    let exp = Exp {node: _Exp::If {
        test: Box::new(Exp {node: _Exp::Int(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: _Exp::String(String::from("perro")), pos: Pos {line: 0, column: 0}})),
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::ThenElseTypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn ifexp_sin_else_no_unit() {
    let exp = Exp {node: _Exp::If {
        test: Box::new(Exp {node: _Exp::Int(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::Int(1), pos: Pos {line: 0, column: 0}}),
        else_: None
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonUnitBody(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn whileexp_ok() {
    let exp = Exp {node: _Exp::While {
        test: Box::new(Exp {node: _Exp::Int(0), pos: Pos {line: 0, column: 0}}),
        body: Box::new(Exp {node: _Exp::Unit, pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn whileexp_condicion_no_entera() {
    let exp = Exp {node: _Exp::While {
        test: Box::new(Exp {node: _Exp::Unit, pos: Pos {line: 0, column: 0}}),
        body: Box::new(Exp {node: _Exp::Unit, pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerCondition(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn forexp_ok() {
    let exp = Exp {node: _Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::Int(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::Int(10), pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::Unit, pos: Pos {line: 0, column: 0,}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn forexp_iterador_es_usable() {
    let exp = Exp {node: _Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::Int(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::Int(10), pos: Pos {line: 0, column: 0,}}),
        body: boxed_exp(_Exp::Seq(vec![possed_exp(_Exp::Var(Var::Simple(Symbol::from("i")))), possed_exp(_Exp::Unit)])),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn forexp_body_no_es_unit() {
    let exp = Exp {node: _Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::Int(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::Int(10), pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::Int(2), pos: Pos {line: 0, column: 0,}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonUnitBody(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn forexp_lo_no_es_int() {
    let exp = Exp {node: _Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::Unit, pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::Int(10), pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::Unit, pos: Pos {line: 0, column: 0,}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerForRange(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn forexp_hi_no_es_int() {
    let exp = Exp {node: _Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::Int(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::Unit, pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::Unit, pos: Pos {line: 0, column: 0,}}),
    }, pos: Pos {line: 0, column: 0}};
        let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerForRange(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn letexp_vardec_sin_tipo_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                None,
                boxed_exp(_Exp::Int(4))
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("foo"))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_vardec_con_tipo_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("int")),
                boxed_exp(_Exp::Int(4)),
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("foo"))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();

    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_vardec_tipo_no_esta_declarado() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("un_tipo_no_declarado")),
                boxed_exp(_Exp::Int(4)),
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn letexp_vardec_tipos_distintos() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("string")),
                boxed_exp(_Exp::Int(4))
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn letexp_typedec_name_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Name(Symbol::from("int"))
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_exp(_Exp::Int(4))
                ),
                Pos{line: 0, column: 0}
            ),
        ],
        body: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("foo"))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_typedec_array_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Array(Symbol::from("int"))
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_exp(_Exp::Array {
                        typ: Symbol::from("FooType"),
                        size:boxed_exp(_Exp::Int(1)),
                        init: boxed_exp(_Exp::Int(2)),
                    })
                ),
                Pos{line: 0, column: 0}
            ),
        ],
        body: boxed_exp(_Exp::Var(
            Var::Subscript(
                Box::new(Var::Simple(Symbol::from("foo"))),
                boxed_exp(_Exp::Int(0))
            )
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_typedec_record_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Record(vec![
                        Field {
                            name: Symbol::from("bar"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }
                    ])
                ),
                Pos{line: 0, column: 1}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_exp(_Exp::Record {
                        fields: vec![(Symbol::from("bar"), boxed_exp(_Exp::Int(1)))],
                        typ: Symbol::from("FooType"),
                    })
                ),
                Pos{line: 0, column: 2}
            )],
        body: boxed_exp(_Exp::Var(
            Var::Field(
                Box::new(Var::Simple(Symbol::from("foo"))),
                Symbol::from("bar")
            )
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_typedec_recursion_infinita() {
   let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("FooType"), Ty::Name(Symbol::from("BaazType"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("BaazType"), Ty::Name(Symbol::from("FooType"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TypeCycle(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}
#[test]
fn test_recursive_ok() {
   let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("C"), Ty::Name(Symbol::from("B"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("B"), Ty::Name(Symbol::from("A"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("A"), Ty::Name(Symbol::from("int"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(..) => panic!("wrong type"),
        Err(..) => panic!("type error"),
    }
}

#[test]
fn letexp_typedec_referencia_tipo_inexistente() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::TypeDec(vec![(
            _TypeDec::new(
                Symbol::from("FooType"),
                Ty::Name(Symbol::from("BaazType"))
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn record_type_cycle_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("List"),
                    Ty::Record(vec![
                        Field {
                            name: Symbol::from("head"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        },
                        Field {
                            name: Symbol::from("tail"),
                            typ: Ty::Name(Symbol::from("List")),
                            escape: false,
                        }
                    ])
                ),
                Pos{line: 0, column: 1}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("List")),
                    boxed_exp(_Exp::Record {
                        fields: vec![
                            (Symbol::from("head"), boxed_exp(_Exp::Int(1))),
                            (Symbol::from("tail"), boxed_exp(_Exp::Record {
                                fields: vec![
                                    (Symbol::from("head"), boxed_exp(_Exp::Int(2))),
                                    (Symbol::from("tail"), boxed_exp(_Exp::Record {
                                        fields: vec![
                                            (Symbol::from("head"), boxed_exp(_Exp::Int(3))),
                                            (Symbol::from("tail"), boxed_exp(_Exp::Record {
                                                fields: vec![
                                                    (Symbol::from("head"), boxed_exp(_Exp::Int(4))),
                                                    (Symbol::from("tail"), boxed_exp(_Exp::Nil))
                                                ],
                                                typ: Symbol::from("List"),
                                            }))
                                        ],
                                        typ: Symbol::from("List"),
                                    }))
                                ],
                                typ: Symbol::from("List"),
                            }))
                        ],
                        typ: Symbol::from("List"),
                    })
                ),
                Pos{line: 0, column: 2}
            )],
        body: boxed_exp(_Exp::Var(
            Var::Field(
                Box::new(Var::Simple(Symbol::from("foo"))),
                Symbol::from("head")
            )
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_functiondec_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::Unit)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_functiondec_llamada_en_bloque_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg1")))),
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("baaz"),
                    vec![Field {
                        name: Symbol::from("arg2"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Call {
                        func: Symbol::from("foo"),
                        args: vec![possed_exp(_Exp::Var(Var::Simple(Symbol::from("arg2"))))],
                    })
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::Int(2))]
        })
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_functiondec_body_no_tipa() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::Var(Var::Simple(Symbol::from("baaz")))), // undeclared
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn letexp_functiondec_body_distinto_result() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::Int(2)),
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn letexp_functiondec_params_repetidos() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::Unit)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_functiondec_nombres_repetidos() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::Unit)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_functiondec_recursivas() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::Unit)
            ),
            Pos{line: 0, column: 0})])],
        body: boxed_exp(_Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn letexp_todas_las_decs_ok() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Name(Symbol::from("int")),
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_exp(_Exp::Int(4))
                ),
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("baaz"),
                    vec![Field {
                        name: Symbol::from("bar"),
                        typ: Ty::Name(Symbol::from("FooType")),
                        escape: false,
                    }],
                    Some(Symbol::from("FooType")),
                    boxed_exp(_Exp::Var(Var::Simple(Symbol::from("bar"))))
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::Var(Var::Simple(Symbol::from("foo"))))]
        })
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(TigerType::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}