use std::fs::{read_dir, read_to_string};
use std::marker::PhantomData;

use super::super::ast::tigerabs::*;
use super::super::ast::position::*;
use super::super::ast::parser::parse;
use super::super::seman::tigerseman::*;
use super::super::seman::escape::*;


fn possed_exp(exp: _Exp) -> Exp {
    Exp {node: exp, pos: Pos {line: 0, column: 0}}
}

fn boxed_exp(exp: _Exp) -> Box<Exp> {
    Box::new(Exp {node: exp, pos: Pos {line: 0, column: 0}})
}

fn decs_different_escapes(decs1: Vec<Dec>, decs2: Vec<Dec>) -> bool {
    fn aux((dec1, dec2) : (&Dec, &Dec)) -> bool {
        match (dec1, dec2) {
            (Dec::VarDec(_VarDec{escape: escape1, ..}, ..), Dec::VarDec(_VarDec{escape: escape2, ..}, ..)) => escape1 == escape2,
            (_, _) => true
        }
    }
    decs1
        .iter()
        .zip(decs2.iter())
        .map(aux)
        .all(|b| b)
}

fn exps_different_escapes(Exp {node: exp1, ..}: Exp, Exp {node: exp2, ..}: Exp) -> bool {
    match (exp1, exp2) {
        (_Exp::ForExp {escape: escape1, ..}, _Exp::ForExp {escape: escape2, ..}) => escape1 != escape2,
        (_Exp::LetExp{decs: decs1, ..}, _Exp::LetExp{decs: decs2, ..}) => decs_different_escapes(decs1, decs2),
        (_Exp::ArrayExp { }, _Exp::ArrayExp { }) =>
        (_Exp::BreakExp, _Exp::BreakExp)
        | (_Exp::IntExp(..), _Exp::IntExp(..))
        | (_Exp::StringExp(..), _Exp::StringExp(..))
        | (_Exp::NilExp, _Exp::NilExp)
        | (_Exp::UnitExp, _Exp::UnitExp) => true,
        (_, _) => panic!("exps differ in structure")
    }
}


#[test]
fn escaped_arguments() -> {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                        phantom: PhantomData
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::LetExp {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                        phantom: PhantomData
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_exp(_Exp::OpExp {
                                        left: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg1")))),
                                        right: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg2")))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_exp(_Exp::CallExp {
                            func: Symbol::from("baaz"),
                            args: vec![possed_exp(_Exp::IntExp(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::CallExp {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::IntExp(2))]
        })
    });
    if let Exp {node: _Exp::LetExp {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::FunctionDec(funs), ..)) = decs.split_first() {
            if let Some(((_FunctionDec{params, ..}, ..), ..)) = funs.split_first() {
                if let Some((Field {escape, ..}, ..)) = params.split_first() {
                    if escape {
                        return () // PASS
                    }
                }
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn not_escaped_arguments() -> {
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                        phantom: PhantomData
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::LetExp {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                        phantom: PhantomData
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_exp(_Exp::OpExp {
                                        left: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg2")))),
                                        right: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg2")))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_exp(_Exp::CallExp {
                            func: Symbol::from("baaz"),
                            args: vec![possed_exp(_Exp::IntExp(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::CallExp {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::IntExp(2))]
        })
    });
    if let Exp {node: _Exp::LetExp {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::FunctionDec(funs), ..)) = decs.split_first() {
            if let Some(((_FunctionDec{params, ..}, ..), ..)) = funs.split_first() {
                if let Some((Field {escape, ..}, ..)) = params.split_first() {
                    if !escape {
                        return () // PASS
                    }
                }
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn escaped_for() {
    // TODO
    let exp = possed_exp(_Exp::LetExp {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                        phantom: PhantomData
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::LetExp {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                        phantom: PhantomData
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_exp(_Exp::OpExp {
                                        left: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg1")))),
                                        right: boxed_exp(_Exp::VarExp(Var::SimpleVar(Symbol::from("arg2")))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_exp(_Exp::CallExp {
                            func: Symbol::from("baaz"),
                            args: vec![possed_exp(_Exp::IntExp(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::CallExp {
            func: Symbol::from("baaz"),
            args: vec![possed_exp(_Exp::IntExp(2))]
        })
    });
    if let Exp {node: _Exp::LetExp {decs, ..}, ..} = find_escapes(exp) {

    }
}