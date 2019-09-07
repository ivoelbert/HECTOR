use std::fs::{read_dir, read_to_string};

use super::super::ast::tigerabs::*;
use super::super::ast::position::*;
use super::super::ast::parser::parse;

use super::super::seman::tigerseman::*;


#[test]
fn test_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let exp = parse(contents).expect("falla el parser");
        let type_env = TypeEnviroment::new();
        let value_env = ValueEnviroment::new();
        type_exp(&exp , &type_env, &value_env).expect("{:?} deberia tipar bien pero falla"/*, path*/);
    }
}

#[test]
fn test_type() {
    let syntax_path = "./tiger_sources/type/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let exp = parse(contents).expect("falla el parser");
        let type_env = TypeEnviroment::new();
        let value_env = ValueEnviroment::new();
        let typed = type_exp(&exp , &type_env, &value_env);
        match typed {
            Err(_) => (),
            Ok(_) => panic!("{:?} deberia fallar pero tipa bien", path),
        }
    }
}

fn possed_exp(exp: _Exp) -> Exp {
    Exp {node: exp, pos: Pos {line: 0, column: 0}}
}

fn boxed_exp(exp: _Exp) -> Box<Exp> {
    Box::new(Exp {node: exp, pos: Pos {line: 0, column: 0}})
}

#[test]
fn test_tipado_unitexp() {
    let exp = Exp {
        node: _Exp::UnitExp,
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(..) => panic!("resultado incorrecto"),
        Err(..) => panic!("Unit tipa mal")
    }
}

#[test]
fn test_tipado_nilexp() {
    let exp = Exp {
        node: _Exp::NilExp,
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TNil) => (),
        Ok(..) => panic!("resultado incorrecto"),
        Err(..) => panic!("Nil tipa mal")
    }
}

#[test]
fn test_tipado_breakexp() {
    let exp = Exp {node: _Exp::BreakExp, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(..) => panic!("resultado incorrecto"),
        Err(..) => panic!("breakexp tipa mal")
    }
}

#[test]
fn test_tipado_intexp() {
    let exp = Exp {
        node: _Exp::IntExp(1),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_stringexp() {
    let exp = Exp {
        node: _Exp::StringExp(String::from("lorem ipsum")),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TString) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_varexp_simplevar_ok() {
    let exp = Exp {
        node: _Exp::VarExp(Var::SimpleVar(Symbol::from("foo"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{ty: Tipo::TInt(R::RW), access: Access::InFrame(1), level: 1});
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_varexp_simplevar_no_declarada() {
    let exp = Exp {
        node: _Exp::VarExp(Var::SimpleVar(Symbol::from("foo"))),
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
fn test_tipado_varexp_simplevar_no_es_simple() {
    let exp = Exp {
        node: _Exp::VarExp(Var::SimpleVar(Symbol::from("f"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        label: String::from("f"),
        formals: vec![],
        result: Tipo::TUnit,
        // level: 0,
        external: false,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn test_tipado_varexp_fieldvar_ok() {
    let exp = Exp {
        node: _Exp::VarExp(Var::FieldVar(Box::new(Var::SimpleVar(Symbol::from("foo"))),Symbol::from("bar"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Tipo::TRecord(
            vec![(Box::new(String::from("bar")),
                Box::new(Tipo::TInt(R::RW)),
                0)], TypeId::new());
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_varexp_fieldvar_field_inexistente() {
    let exp = Exp {
        node: _Exp::VarExp(Var::FieldVar(Box::new(Var::SimpleVar(Symbol::from("foo"))),Symbol::from("perro"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Tipo::TRecord(
            vec![(Box::new(String::from("bar")),
                Box::new(Tipo::TInt(R::RW)),
                0)],
                TypeId::new(),
            );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
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
fn test_tipado_varexp_fieldvar_sobre_tipo_no_record() {
    let exp = Exp {
        node: _Exp::VarExp(Var::FieldVar(Box::new(Var::SimpleVar(Symbol::from("foo"))),Symbol::from("bar"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Tipo::TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
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
fn test_tipado_varexp_subscriptvar_ok() {
    let exp = Exp {
        node: _Exp::VarExp(
            Var::SubscriptVar(Box::new(Var::SimpleVar(Symbol::from("foo"))),
            Box::new(Exp {
                node: _Exp::IntExp(0),
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
    let foo_type = Tipo::TArray(
        Box::new(Tipo::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_varexp_subscriptvar_indice_no_entero() {
    let exp = Exp {
        node: _Exp::VarExp(
            Var::SubscriptVar(Box::new(Var::SimpleVar(Symbol::from("foo"))),
            Box::new(Exp {
                node: _Exp::StringExp(String::from("una string de indice :o")),
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
    let foo_type = Tipo::TArray(
        Box::new(Tipo::TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
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
fn test_tipado_varexp_subscriptvar_no_array() {
    let exp = Exp {
        node: _Exp::VarExp(
            Var::SubscriptVar(Box::new(Var::SimpleVar(Symbol::from("foo"))),
            Box::new(Exp {
                node: _Exp::IntExp(0),
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
    let foo_type = Tipo::TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
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
fn test_tipado_callexp_ok() {
    let exp = Exp {
        node: _Exp::CallExp {
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
        label: String::from("f"),
        formals: vec![],
        result: Tipo::TUnit,
        // level: 0,
        external: false,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_callexp_args_de_mas() {
    let exp = Exp {
        node: _Exp::CallExp {
            func: Symbol::from("f"),
            args: vec![Exp {
                node: _Exp::IntExp(1),
                pos: Pos {line: 0, column: 0}
            }],
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        label: String::from("f"),
        formals: vec![],
        result: Tipo::TUnit,
        // level: 0,
        external: true,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TooManyArguments(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn test_tipado_callexp_args_de_menos() {
    let exp = Exp {
        node: _Exp::CallExp {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        label: String::from("f"),
        formals: vec![Tipo::TInt(R::RW)],
        result: Tipo::TUnit,
        // level: 0,
        external: true,
    });
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::TooFewArguments(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn test_tipado_callexp_funcion_no_declarada() {
    let exp = Exp {
        node: _Exp::CallExp {
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
fn test_tipado_opexp_ok() {
    let exp = Exp {
        node: _Exp::OpExp {
            left: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
            oper: Oper::PlusOp,
            right: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_opexp_tipos_distintos() {
    let exp = Exp {
        node: _Exp::OpExp {
            left: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
            oper: Oper::PlusOp,
            right: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
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
fn test_tipado_recordexp_ok() {
    let exp = Exp {
        node: _Exp::RecordExp {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Tipo::TRecord(
            vec![(Box::new(String::from("baz")),
                Box::new(Tipo::TInt(R::RW)),
                0)], TypeId::new());
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(return_type) => assert!(return_type == foo_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_recordexp_tipo_inexistente() {
    let exp = Exp {
        node: _Exp::RecordExp {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}))],
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
fn test_tipado_recordexp_con_tipo_no_record() {
    let exp = Exp {
        node: _Exp::RecordExp {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    type_env.insert(Symbol::from("FooType"), Tipo::TInt(R::RW));
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotRecordType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn test_tipado_arrayexp_ok() {
    let exp = Exp {node: _Exp::ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::IntExp(2), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Tipo::TArray(
        Box::new(Tipo::TInt(R::RW)),
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
fn test_tipado_arrayexp_size_no_int() {
    let exp = Exp {node: _Exp::ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Tipo::TArray(
        Box::new(Tipo::TInt(R::RW)),
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
fn test_tipado_arrayexp_tipos_distintos() {
    let exp = Exp {node: _Exp::ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Tipo::TArray(
        Box::new(Tipo::TInt(R::RW)),
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
fn test_tipado_arrayexp_tipo_no_array() {
    let exp = Exp {node: _Exp::ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Tipo::TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Err(TypeError::NotArrayType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
fn test_tipado_arrayexp_tipo_no_existe() {
    let exp = Exp {node: _Exp::ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
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
fn test_tipado_seqexp_ok() {
    let exp = Exp {
        node: _Exp::SeqExp(vec![
            Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}},
            Exp {node: _Exp::IntExp(2), pos: Pos {line: 0, column: 0}},
        ]),
        pos: Pos {line: 0, column: 0}
    };
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}
// Se puede testear algo mas de _Exp::SeqExp? Hay alguna condicion del ultimo tipo?

#[test]
fn test_tipado_assignexp_ok() {
    let exp = Exp {node: _Exp::AssignExp{
        var: Var::SimpleVar(Symbol::from("foo")),
        exp: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: Tipo::TInt(R::RW),
        access: Access::InFrame(1),
        level: 1,
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_assignexp_variable_no_existe() {
    let exp = Exp {node: _Exp::AssignExp{
        var: Var::SimpleVar(Symbol::from("foo")),
        exp: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
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
fn test_tipado_assignexp_tipos_distintos() {
    let exp = Exp {node: _Exp::AssignExp{
        var: Var::SimpleVar(Symbol::from("foo")),
        exp: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: Tipo::TInt(R::RW),
        access: Access::InFrame(1),
        level: 1,
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
fn test_tipado_assignexp_variable_read_only() {
    let exp = Exp {node: _Exp::AssignExp{
        var: Var::SimpleVar(Symbol::from("i")),
        exp: Box::new(Exp {node: _Exp::IntExp(2), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: Tipo::TInt(R::RO),
        access: Access::InFrame(1),
        level: 1,
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
fn test_tipado_ifexp_ok() {
    let exp = Exp {node: _Exp::IfExp {
        test: Box::new(Exp {node: _Exp::IntExp(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: _Exp::IntExp(2), pos: Pos {line: 0, column: 0}}))
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(_)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_ifexp_test_no_entero() {
    let exp = Exp {node: _Exp::IfExp {
        test: Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: _Exp::IntExp(2), pos: Pos {line: 0, column: 0}}))
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
fn test_tipado_ifexp_tipos_then_else_distintos() {
    let exp = Exp {node: _Exp::IfExp {
        test: Box::new(Exp {node: _Exp::IntExp(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: _Exp::StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}})),
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
fn test_tipado_ifexp_sin_else_no_unit() {
    let exp = Exp {node: _Exp::IfExp {
        test: Box::new(Exp {node: _Exp::IntExp(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0}}),
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
fn test_tipado_whileexp_ok() {
    let exp = Exp {node: _Exp::WhileExp {
        test: Box::new(Exp {node: _Exp::IntExp(0), pos: Pos {line: 0, column: 0}}),
        body: Box::new(Exp {node: _Exp::UnitExp, pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_whileexp_condicion_no_entera() {
    let exp = Exp {node: _Exp::WhileExp {
        test: Box::new(Exp {node: _Exp::UnitExp, pos: Pos {line: 0, column: 0}}),
        body: Box::new(Exp {node: _Exp::UnitExp, pos: Pos {line: 0, column: 0}}),
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
fn test_tipado_forexp_ok() {
    let exp = Exp {node: _Exp::ForExp {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::IntExp(10), pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::UnitExp, pos: Pos {line: 0, column: 0,}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_forexp_iterador_es_usable() {
    let exp = Exp {node: _Exp::ForExp {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::IntExp(10), pos: Pos {line: 0, column: 0,}}),
        body: boxed_exp(_Exp::SeqExp(vec![possed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("i")))), possed_exp(_Exp::UnitExp)])),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_forexp_body_no_es_unit() {
    let exp = Exp {node: _Exp::ForExp {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::IntExp(10), pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::IntExp(2), pos: Pos {line: 0, column: 0,}}),
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
fn test_tipado_forexp_lo_no_es_int() {
    let exp = Exp {node: _Exp::ForExp {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::UnitExp, pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::IntExp(10), pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::UnitExp, pos: Pos {line: 0, column: 0,}}),
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
fn test_tipado_forexp_hi_no_es_int() {
    let exp = Exp {node: _Exp::ForExp {
        var: Symbol::from("i"),
        escape: false,
        lo: Box::new(Exp { node: _Exp::IntExp(1), pos: Pos {line: 0, column: 0,}}),
        hi: Box::new(Exp { node: _Exp::UnitExp, pos: Pos {line: 0, column: 0,}}),
        body: Box::new(Exp { node: _Exp::UnitExp, pos: Pos {line: 0, column: 0,}}),
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
fn test_tipado_letexp_vardec_sin_tipo_ok() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                None,
                boxed_exp(_Exp::IntExp(4))
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("foo"))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_vardec_con_tipo_ok() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("int")),
                boxed_exp(_Exp::IntExp(4)),
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("foo"))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();

    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_vardec_tipo_no_esta_declarado() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("un_tipo_no_declarado")),
                boxed_exp(_Exp::IntExp(4)),
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::UnitExp)
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
fn test_tipado_letexp_vardec_tipos_distintos() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("string")),
                boxed_exp(_Exp::IntExp(4))
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_exp(_Exp::UnitExp)
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
fn test_tipado_letexp_typedec_name_ok() {
    let exp = possed_exp(_Exp::LetExp {
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
                    boxed_exp(_Exp::IntExp(4))
                ),
                Pos{line: 0, column: 0}
            ),
        ],
        body: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("foo"))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_typedec_array_ok() {
    let exp = possed_exp(_Exp::LetExp {
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
                    boxed_exp(_Exp::ArrayExp {
                        typ: Symbol::from("FooType"),
                        size:boxed_exp(_Exp::IntExp(1)),
                        init: boxed_exp(_Exp::IntExp(2)),
                    })
                ),
                Pos{line: 0, column: 0}
            ),
        ],
        body: boxed_exp(_Exp::VarExp(
            Var::SubscriptVar(
                Box::new(Var::SimpleVar(Symbol::from("foo"))),
                boxed_exp(_Exp::IntExp(0))
            )
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_typedec_record_ok() {
    let exp = possed_exp(_Exp::LetExp {
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
                    boxed_exp(_Exp::RecordExp {
                        fields: vec![(Symbol::from("bar"), boxed_exp(_Exp::IntExp(1)))],
                        typ: Symbol::from("FooType"),
                    })
                ),
                Pos{line: 0, column: 2}
            )],
        body: boxed_exp(_Exp::VarExp(
            Var::FieldVar(
                Box::new(Var::SimpleVar(Symbol::from("foo"))),
                Symbol::from("bar")
            )
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_typedec_recursion_infinita() {
   let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("FooType"), Ty::Name(Symbol::from("BaazType"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("BaazType"), Ty::Name(Symbol::from("FooType"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_exp(_Exp::UnitExp)
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
fn test_typecheck_recursive_typecheck_ok() {
   let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("C"), Ty::Name(Symbol::from("B"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("B"), Ty::Name(Symbol::from("A"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("A"), Ty::Name(Symbol::from("int"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_exp(_Exp::UnitExp)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(..) => panic!("wrong type"),
        Err(..) => panic!("type error"),
    }
}

#[test]
fn test_tipado_letexp_typedec_referencia_tipo_inexistente() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::TypeDec(vec![(
            _TypeDec::new(
                Symbol::from("FooType"),
                Ty::Name(Symbol::from("BaazType"))
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::UnitExp)
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
fn test_tipado_letexp_functiondec_ok() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false
                }],
                None,
                boxed_exp(_Exp::UnitExp)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::UnitExp)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_functiondec_llamada_en_bloque_ok() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg1")))),
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("baaz"),
                    vec![Field {
                        name: Symbol::from("arg2"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::CallExp {
                        func: Symbol::from("foo"),
                        args: vec![possed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg2"))))],
                    })
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::CallExp {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::IntExp(2))]
        })
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TInt(R::RW)) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_functiondec_body_no_tipa() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("baaz")))), // no declarada,
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::UnitExp)
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
fn test_tipado_letexp_functiondec_body_distinto_result() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false
                }],
                None,
                boxed_exp(_Exp::IntExp(2)), // no declarada,
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::UnitExp)
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
fn test_tipado_letexp_functiondec_params_repetidos() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false
                }],
                None,
                boxed_exp(_Exp::UnitExp)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::UnitExp)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_functiondec_nombres_repetidos() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false
                }],
                None,
                boxed_exp(_Exp::UnitExp)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_exp(_Exp::UnitExp)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_functiondec_recursivas() {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false
                }],
                None,
                boxed_exp(_Exp::UnitExp)
            ),
            Pos{line: 0, column: 0})])],
        body: boxed_exp(_Exp::UnitExp)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env);
    match res {
        Ok(Tipo::TUnit) => (),
        Ok(tiger_type) => panic!("wrong type: {:?}", tiger_type),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
fn test_tipado_letexp_todas_las_decs_ok() {
    let exp = possed_exp(_Exp::LetExp {
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
                    boxed_exp(_Exp::IntExp(4))
                ),
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("baaz"),
                    vec![Field {
                        name: Symbol::from("bar"),
                        typ: Ty::Name(Symbol::from("FooType")),
                        escape: false
                    }],
                    Some(Symbol::from("FooType")),
                    boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("bar"))))
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::CallExp {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("foo"))))]
        })
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(&exp, &type_env, &value_env)
        .expect("no puedo tipar un let que usa las declaraciones");
    assert_eq!(res, Tipo::TInt(R::RW))
}