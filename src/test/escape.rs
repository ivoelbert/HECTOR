use crate::ast::*;
use crate::ast::position::*;
use crate::tree::escape::*;

#[test]
fn escaped_arguments() {
    let exp = make_ast(Exp::Let {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Let {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_ast(Exp::Op {
                                        left: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg1"))))),
                                        right: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg2"))))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_ast(Exp::Call {
                            func: Symbol::from("baaz"),
                            args: vec![make_ast(Exp::Int(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_ast(Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![make_ast(Exp::Int(2))]
        })
    });
    if let AST {node: Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::FunctionDec(funs), ..)) = decs.split_first() {
            if let Some(((_FunctionDec{params, ..}, ..), ..)) = funs.split_first() {
                if let Some((Field {escape, ..}, ..)) = params.split_first() {
                    if *escape {
                        return () // PASS
                    } else {
                        panic!("wrong escape")
                    }
                }
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn not_escaped_arguments() {
    let exp = make_ast(Exp::Let {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Let {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_ast(Exp::Op {
                                        left: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg2"))))),
                                        right: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg2"))))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_ast(Exp::Call {
                            func: Symbol::from("baaz"),
                            args: vec![make_ast(Exp::Int(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_ast(Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![make_ast(Exp::Int(2))]
        })
    });
    if let AST {node: Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::FunctionDec(funs), ..)) = decs.split_first() {
            if let Some(((_FunctionDec{params, ..}, ..), ..)) = funs.split_first() {
                if let Some((Field {escape, ..}, ..)) = params.split_first() {
                    if !escape {
                        return () // PASS
                    } else {
                        panic!("wrong escape")
                    }
                }
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn escaped_var() {
    let exp = make_ast(Exp::Let {
        decs: vec![
            Dec::VarDec(
                _VarDec{name: Symbol::from("var1"), escape: false, init: boxed_ast(Exp::Int(1)), typ: None}, // var defined here
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("var1"))))), // and used here
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_ast(Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![make_ast(Exp::Int(2))]
        })
    });
    if let AST {node: Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::VarDec(_VarDec{escape, ..}, ..), ..)) = decs.split_first() {
            if *escape {
                return () // PASS
            } else {
                panic!("wrong escape")
            }
        }
    }
    panic!("wrong structure")
}
#[test]
fn not_escaped_var() {
    let exp = make_ast(Exp::Let {
        decs: vec![
            Dec::VarDec(
                _VarDec{name: Symbol::from("var1"), escape: false, init: boxed_ast(Exp::Int(1)), typ: None}, // var defined, never used
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"), // arg defined here
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg1"))))),  // and used here
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_ast(Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![make_ast(Exp::Int(2))]
        })
    });
    if let AST {node: Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::VarDec(_VarDec{escape, ..}, ..), ..)) = decs.split_first() {
            if !*escape {
                return () // PASS
            } else {
                panic!("wrong escape")
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn escaped_for() {
    // TODO
    let exp = make_ast(Exp::For {
        var: Symbol::from("i"), // iterator defined here
        lo: boxed_ast(Exp::Int(1)),
        hi: boxed_ast(Exp::Int(1)),
        body: boxed_ast(Exp::Let {
            decs: vec![
                Dec::FunctionDec(vec![(
                    _FunctionDec::new(
                        Symbol::from("fun1"),
                        vec![Field {
                            name: Symbol::from("arg1"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }],
                        Some(Symbol::from("int")),
                        boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("i"))))), // and used here
                    ),
                    Pos{line: 0, column: 0}
                )]),
            ],
            body: boxed_ast(Exp::Call {
                func: Symbol::from("fun1"),
                args: vec![make_ast(Exp::Int(2))]
            })
        }),
        escape: false
    });
    if let AST {node: Exp::For {escape, ..}, ..} = find_escapes(exp) {
        assert!(escape)
    }
}
#[test]
fn not_escaped_for() {
    // TODO
    let exp = make_ast(Exp::For {
        var: Symbol::from("i"), // iterator defined here
        lo: boxed_ast(Exp::Int(1)),
        hi: boxed_ast(Exp::Int(1)),
        body: boxed_ast(Exp::Let {
            decs: vec![
                Dec::FunctionDec(vec![(
                    _FunctionDec::new(
                        Symbol::from("fun1"),
                        vec![Field {
                            name: Symbol::from("arg1"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }],
                        Some(Symbol::from("int")),
                        boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg1"))))), // but not used
                    ),
                    Pos{line: 0, column: 0}
                )]),
            ],
            body: boxed_ast(Exp::Call {
                func: Symbol::from("fun1"),
                args: vec![make_ast(Exp::Int(2))]
            })
        }),
        escape: false
    });
    if let AST {node: Exp::For {escape, ..}, ..} = find_escapes(exp) {
        assert!(!escape)
    }
}