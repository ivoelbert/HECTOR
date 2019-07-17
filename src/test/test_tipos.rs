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
fn test_tipado_breakexp() {
    let exp = Exp {node: BreakExp, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("breakexp tipa mal")
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
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{ty: TInt(R::RW), access: Access::InFrame(1), level: 1});
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
fn test_tipado_varexp_simplevar_no_es_simple() {
    let exp = Exp {
        node: VarExp(SimpleVar(Symbol::from("f"))),
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
        Err(NotSimpleVar(_)) => assert!(true),
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
    let foo_type = TRecord(
            vec![(Box::new(String::from("bar")),
                Box::new(TInt(R::RW)),
                0)], TypeId::new());
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TInt(R::RW)) => assert!(true),
        _ => panic!("fieldvar esta tipando mal")
    }
}

#[test]
fn test_tipado_varexp_fieldvar_field_inexistente() {
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
            vec![(Box::new(String::from("bar")),
                Box::new(TInt(R::RW)),
                0)], 
                TypeId::new(),
            );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
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
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
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
    let foo_type = TArray(
        Box::new(TInt(R::RW)), 
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
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
    let foo_type = TArray(
        Box::new(TInt(R::RW)),
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
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
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        access: Access::InFrame(0),
        level: 0,
        ty: foo_type,
    });
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NotArrayType(_)) => assert!(true),
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
    let foo_type = TRecord(
            vec![(Box::new(String::from("bar")),
                Box::new(TInt(R::RW)),
                0)], TypeId::new());
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(return_type) => assert!(return_type == foo_type),
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
    type_env.insert(Symbol::from("FooType"), TInt(R::RW));
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NotRecordType(_)) => assert!(true),
        _ => panic!("podes darle a un record un tipo que no es record"),
    }
}

#[test]
fn test_tipado_arrayexp_ok() {
    let exp = Exp {node: ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: IntExp(2), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let foo_type = TArray(
        Box::new(TInt(R::RW)), 
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(return_type) => assert!(return_type == foo_type),
        _ => panic!("array")
    }
}

#[test]
fn test_tipado_arrayexp_size_no_int() {
    let exp = Exp {node: ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let foo_type = TArray(
        Box::new(TInt(R::RW)), 
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NonIntegerSize(_)) => assert!(true),
        _ => panic!("podemos tipar array de tamaño perro")
    }
}

#[test]
fn test_tipado_arrayexp_tipos_distintos() {
    let exp = Exp {node: ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let foo_type = TArray(
        Box::new(TInt(R::RW)), 
        TypeId::new(),
    );
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(TypeMismatch(_)) => assert!(true),
        _ => panic!("un array inicializado con algo de un tipo distinto tipa")
    }
}

#[test]
fn test_tipado_arrayexp_tipo_no_array() {
    let exp = Exp {node: ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let mut type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let foo_type = TInt(R::RW);
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NotArrayType(_)) => assert!(true),
        _ => panic!("un array inicializado con algo de un tipo distinto tipa")
    }
}

#[test]
fn test_tipado_arrayexp_tipo_no_existe() {
    let exp = Exp {node: ArrayExp {
        typ: Symbol::from("FooType"),
        size: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        init: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(UndeclaredType(_)) => assert!(true),
        _ => panic!("un array de tipo que no existe tipa")
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

#[test]
fn test_tipado_assignexp_ok() {
    let exp = Exp {node: AssignExp{
        var: SimpleVar(Symbol::from("foo")),
        exp: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let env_entry = EnvEntry::Var{
        ty: TInt(R::RW),
        access: Access::InFrame(1),
        level: 1,
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("assignexp tipa mal")
    }
}

#[test]
fn test_tipado_assignexp_variable_no_existe() {
    let exp = Exp {node: AssignExp{
        var: SimpleVar(Symbol::from("foo")),
        exp: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(UndeclaredSimpleVar(_)) => assert!(true),
        _ => panic!("Podes asginar a una variable no declarada")
    }
}

#[test]
fn test_tipado_assignexp_tipos_distintos() {
    let exp = Exp {node: AssignExp{
        var: SimpleVar(Symbol::from("foo")),
        exp: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let env_entry = EnvEntry::Var{
        ty: TInt(R::RW),
        access: Access::InFrame(1),
        level: 1,
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(TypeMismatch(_)) => assert!(true),
        _ => panic!("Podes asginar a una variable un valor de otro tipo")
    }
}

#[test]
fn test_tipado_assignexp_variable_read_only() {
    let exp = Exp {node: AssignExp{
        var: SimpleVar(Symbol::from("i")),
        exp: Box::new(Exp {node: IntExp(2), pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let env_entry = EnvEntry::Var{
        ty: TInt(R::RO),
        access: Access::InFrame(1),
        level: 1,
    };
    value_env.insert(Symbol::from("i"), env_entry);
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(ReadOnlyAssignment(_)) => assert!(true),
        _ => panic!("Podes asginar a una variable read only")
    }
}

#[test]
fn test_tipado_ifexp_ok() {
    let exp = Exp {node: IfExp {
        test: Box::new(Exp {node: IntExp(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: IntExp(2), pos: Pos {line: 0, column: 0}}))
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("ifexp esta tipando mal")
    }
}

#[test]
fn test_tipado_ifexp_test_no_entero() {
    let exp = Exp {node: IfExp {
        test: Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: IntExp(2), pos: Pos {line: 0, column: 0}}))
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NonIntegerCondition(_)) => assert!(true),
        _ => panic!("puede tener un if con condicion no entera"),
    }
}

#[test]
fn test_tipado_ifexp_tipos_then_else_distintos() {
    let exp = Exp {node: IfExp {
        test: Box::new(Exp {node: IntExp(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: Some(Box::new(Exp {node: StringExp(String::from("perro")), pos: Pos {line: 0, column: 0}})),
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(ThenElseTypeMismatch(_)) => assert!(true),
        _ => panic!("puedo tener un if con tipos distintos en then y else"),
    }
}

#[test]
fn test_tipado_ifexp_sin_else_no_unit() {
    let exp = Exp {node: IfExp {
        test: Box::new(Exp {node: IntExp(0), pos: Pos {line: 0, column: 0}}),
        then_: Box::new(Exp {node: IntExp(1), pos: Pos {line: 0, column: 0}}),
        else_: None
    }
    , pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NonUnitBody(_)) => assert!(true),
        _ => panic!("puedo tener un if sin else y con then no TUnit"),
    }
}

#[test]
fn test_tipado_whileexp_ok() {
    let exp = Exp {node: WhileExp {
        test: Box::new(Exp {node: IntExp(0), pos: Pos {line: 0, column: 0}}),
        body: Box::new(Exp {node: UnitExp, pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Ok(TUnit) => assert!(true),
        _ => panic!("whileexp tipa mal")
    }
}

#[test]
fn test_tipado_whileexp_condicion_no_entera() {
    let exp = Exp {node: WhileExp {
        test: Box::new(Exp {node: UnitExp, pos: Pos {line: 0, column: 0}}),
        body: Box::new(Exp {node: UnitExp, pos: Pos {line: 0, column: 0}}),
    }, pos: Pos {line: 0, column: 0}};
    let type_env = TypeEnviroment::new();
    let value_env = ValueEnviroment::new();
    let res = tipar_exp(exp, type_env, value_env);
    match res {
        Err(NonIntegerCondition(_)) => assert!(true),
        _ => panic!("podes tener un while con condicion no entera")
    }
}


// Estos tests los dejo comentado porque se que es lo que hay que testear, pero todavia no entiendo muy bien esas
// structs y no se bien como testearlo.

// #[test]
// fn test_tipado_forexp_ok() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_forexp_body_no_es_unit() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_forexp_lo_no_es_int() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_forexp_hi_no_es_int() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_vardec_ok() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_vardec_tipo_en_esta_dec() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_vardec_tipos_distintos() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_vardec_tipo_inexistente() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_vardec_variables_repetidas() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_typedec_ok() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_typedec_recursion_infinita() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_typedec_referencia_tipo_inexistente() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_letexp_typedec_referencia_tipos_repetidos() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_functiondec_ok() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_functiondec_nombres_tipo_param_en_esta_dec() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_functiondec_nombres_repetidos() {
//     let exp = Exp {node: UnitExp, pos: Pos {line: 0, column: 0}};
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// no se me ocurre pero se deben poder hacer muchos mas tests con las declaraciones de funciones


