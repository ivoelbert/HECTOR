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
        Err(UndeclaredVar(_)) => assert!(true),
        _ => panic!("Puedo tipar una variable no declarada")
    }
}

#[test]
fn test_tipado_varexp_fieldvar_ok() {
    // Este test no compila por algo que no entiendo de que no se pueden mover las variables.
    // Tipo no puede derivar Copy porque String no puede derivar Copy.
    // Tiene algo que ver con quien se hace cargo de liberar la memoria.
    let exp = Exp {
        node: VarExp(FieldVar(Box::new(SimpleVar(Symbol::from("foo"))),Symbol::from("baz"))),
        pos: Pos {
            line: 0,
            column: 0,
        }
    };
    let mut type_env = TypeEnviroment::new();
    let mut value_env = ValueEnviroment::new();
    let foo_type = TRecord(
            vec![(String::from("baz"),
                TInt(R::RW),
                0)], Box::new(()));
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

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }

// #[test]
// fn test_tipado_unitexp() {
//     let exp = Exp {
//         node: UnitExp,
//         pos: Pos {
//             line: 0,
//             column: 0,
//         }
//     };
//     let type_env = TypeEnviroment::new();
//     let value_env = ValueEnviroment::new();
//     let res = tipar_exp(exp, type_env, value_env);
//     match res {
//         Ok(TUnit) => assert!(true),
//         _ => panic!("")
//     }
// }
