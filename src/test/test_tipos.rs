use std::fs::{read_dir, read_to_string};

use super::super::ast::tigerabs::*;
use super::super::ast::position::*;
use _Exp::*;
use Dec::*;
use Ty::*;
use Var::*;

use super::super::seman::tigerseman::*;
use Tipo::*;
use TypeError::*;


// #[test]
// fn test_good() {
//     let good_path = "./tiger_sources/good/";
//     let source_files = read_dir(good_path).unwrap();
//     for direntry in source_files {
//         let path = direntry.unwrap().path();
//         let mut contents = read_to_string(&path).unwrap();
//         parse(contents).expect("Compilation failed");
//     }
// }


#[test]
fn test_tipado_unitexp() {
    let exp = Exp {
        node: UnitExp,
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("Unit tipa mal")
    }
}

#[test]
fn test_tipado_nilexp() {
    let exp = Exp {
        node: NilExp,
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TNil) => assert!(true),
        _ => panic!("Nil tipa mal")
    }
}

#[test]
fn test_tipado_intexp() {
    let exp = Exp {
        node: IntExp(1),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("IntExp tipa mal")
    }
}

#[test]
fn test_tipado_stringexp() {
    let exp = Exp {
        node: StringExp(String::from("lorem ipsum")),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TString) => assert!(true),
        _ => panic!("StringExp tipa mal")
    }
}

#[test]
fn test_tipado_varexp_simplevar_ok() {
    let exp = Exp {
        node: VarExp(SimpleVar(Symbol::from("foo"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{ty: &TInt(R::RW), access: Access::InFrame(1), level: 1});
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("simplevar tipa mal")
    }
}

#[test]
fn test_tipado_varexp_simplevar_no_declarada() {
    let exp = Exp {
        node: VarExp(SimpleVar(Symbol::from("foo"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(UndeclaredSimpleVar(_)) => assert!(true),
        _ => panic!("Puedo tipar una simplevar no declarada")
    }
}

#[test]
fn test_tipado_varexp_fieldvar_ok() {
    // Este test no compila por algo que no entiendo de que no se pueden mover las variables.
    // Tipo no puede derivar Copy porque String no puede derivar Copy.
    // Tiene algo que ver con quien se hace cargo de liberar la memoria.
    let exp = Exp {
        node: VarExp(FieldVar(Box::new(SimpleVar(Symbol::from("foo"))),Symbol::from("bar"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let nil = ();
    let foo_type = TRecord(
            vec![(String::from("bar"),
                Box::new(TInt(R::RW)),
                0)], &nil);
    type_env.insert(Symbol::from("FooType"), &foo_type);
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: &foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("fieldvar esta tipando mal")
    }
}

#[test]
fn test_tipado_varexp_fieldvar_fields_inexistente() {
    // Este test no compila por algo que no entiendo de que no se pueden mover las variables.
    // Tipo no puede derivar Copy porque String no puede derivar Copy.
    // Tiene algo que ver con quien se hace cargo de liberar la memoria.
    let exp = Exp {
        node: VarExp(FieldVar(Box::new(SimpleVar(Symbol::from("foo"))),Symbol::from("perro"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let unit = ();
    let foo_type = TRecord(
            vec![(String::from("bar"),
                Box::new(TInt(R::RW)),
                0)], &unit);
    type_env.insert(Symbol::from("FooType"), &foo_type);
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: &foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(FieldDoesNotExist(_)) => assert!(true),
        _ => panic!("fieldvar con field inexistente esta tipando mal")
    }
}

#[test]
fn test_tipado_varexp_fieldvar_sobre_tipo_no_record() {
    // Este test no compila por algo que no entiendo de que no se pueden mover las variables.
    // Tipo no puede derivar Copy porque String no puede derivar Copy.
    // Tiene algo que ver con quien se hace cargo de liberar la memoria.
    let exp = Exp {
        node: VarExp(FieldVar(Box::new(SimpleVar(Symbol::from("foo"))),Symbol::from("bar"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let foo_type = TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), &foo_type);
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: &foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NotRecordType(_)) => assert!(true),
        _ => panic!("fieldvar sobre algo que no es record esta tipando mal")
    }
}

#[test]
fn test_tipado_varexp_subscriptvar_ok() {
    let exp = Exp {
        node: VarExp(
            SubscriptVar(Box::new(SimpleVar(Symbol::from("foo"))),
            Box::new(Exp {
                node: IntExp(0),
                pos: Pos {
                    line: 0,
                    column: 0,
            }}))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let unit = ();
    let foo_type = Box::new(TArray(Box::new(TInt(R::RW)), &unit));
    type_env.insert(Symbol::from("FooType"), &foo_type);
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: &foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("subscriptvar esta tipando mal")
    }
}

#[test]
fn test_tipado_varexp_subscriptvar_indice_no_entero() {
    let exp = Exp {
        node: VarExp(
            SubscriptVar(Box::new(SimpleVar(Symbol::from("foo"))),
            Box::new(Exp {
                node: StringExp(String::from("una string de indice :o")),
                pos: Pos {
                    line: 0,
                    column: 0,
            }}))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let unit = ();
    let foo_type = Box::new(TArray(Box::new(TInt(R::RW)), &unit));
    type_env.insert(Symbol::from("FooType"), &foo_type);
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: &foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(SunscriptNotInteger(_)) => assert!(true),
        _ => panic!("puedo tipar un subscript var con indice no entero")
    }
}

#[test]
fn test_tipado_varexp_subscriptvar_no_array() {
    let exp = Exp {
        node: VarExp(
            SubscriptVar(Box::new(SimpleVar(Symbol::from("foo"))),
            Box::new(Exp {
                node: IntExp(0),
                pos: Pos {
                    line: 0,
                    column: 0,
            }}))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let foo_type = TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), &foo_type);
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: &foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NotRecordType(_)) => assert!(true),
        _ => panic!("subscriptvar sobre algo que no es array esta tipando mal")
    }
}

#[test]
fn test_tipado_callexp_ok() {
    let exp = Exp {
        node: CallExp {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        label: String::from("f"),
        formals: vec![],
        result: TUnit,
        // level: 0,
        external: false,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("callexp tipa mal")
    }
}

#[test]
fn test_tipado_callexp_args_de_mas() {
    let exp = Exp {
        node: CallExp {
            func: Symbol::from("f"),
            args: vec![Box::new(Exp {
                node: IntExp(1),
                pos: Pos {line: 0, column: 0}
            })],
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        label: String::from("f"),
        formals: vec![],
        result: TUnit,
        // level: 0,
        external: true,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(TooManyArguments(_)) => assert!(true),
        _ => panic!("una llamada a funcion con argumentos de mas tipa")
    }
}

#[test]
fn test_tipado_callexp_args_de_menos() {
    let exp = Exp {
        node: CallExp {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        label: String::from("f"),
        formals: vec![TInt(R::RW)],
        result: TUnit,
        // level: 0,
        external: true,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(TooFewArguments(_)) => assert!(true),
        _ => panic!("una llamada a funcion con argumentos de menos tipa")
    }
}

#[test]
fn test_tipado_callexp_funcion_no_declarada() {
    let exp = Exp {
        node: CallExp {
            func: Symbol::from("f"),
            args: vec![],
        },
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(UndeclaredFunction(_)) => assert!(true),
        _ => panic!("llamar a una funcion que no existe tipa")
    }
}

#[test]
fn test_tipado_opexp_ok() {
    let exp = Exp {
        node: OpExp {
            left: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
            oper: Oper::PlusOp,
            right: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("opexp tipa mal")
    }
}

#[test]
fn test_tipado_opexp_tipos_distintos() {
    let exp = Exp {
        node: OpExp {
            left: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
            oper: Oper::PlusOp,
            right: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(WrongOperatorTypes(_)) => assert!(true),
        _ => panic!("podes sumar 1 con perro"),
    }
}

#[test]
fn test_tipado_recordexp_ok() {
    let exp = Exp {
        node: RecordExp {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let mut type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let nil = ();
    let foo_type = TRecord(
            vec![(String::from("bar"),
                Box::new(TInt(R::RW)),
                0)], &nil);
    type_env.insert(Symbol::from("FooType"), &foo_type);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("recordexp tipa mal")
    }
}

#[test]
fn test_tipado_recordexp_tipo_inexistente() {
    let exp = Exp {
        node: RecordExp {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(UndeclaredType(_)) => assert!(true),
        _ => panic!("podes darle a un record un tipo que no existe"),
    }
}

#[test]
fn test_tipado_recordexp_con_tipo_no_record() {
    let exp = Exp {
        node: RecordExp {
            fields: vec![(Symbol::from("baz"), Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}))],
            typ: Symbol::from("FooType"),
        },
        pos: Pos {line: 0, column: 0}
    };
    let mut type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    type_env.insert(Symbol::from("FooType"), &TInt(R::RW));
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NotRecordType(_)) => assert!(true),
        _ => panic!("podes darle a un record un tipo que no es record"),
    }
}

#[test]
fn test_tipado_seqexp_ok() {
    let exp = Exp {
        node: SeqExp(vec![
            Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
            Box::new(Exp {node: IntExp(2), pos: Pos {line: 0, column: 0}}),
        ]),
        pos: Pos {line: 0, column: 0}
    };
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("seqexp esta tipando mal")
    }
}
// Se puede testear algo mas de SeqExp? Hay alguna condicion del ultimo tipo?

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }
