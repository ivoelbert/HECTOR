use std::collections::HashMap;
use crate::ast::*;

type EscapeTable = HashMap<Symbol, (u32, bool)>;

fn trav_var(Var{kind, typ, pos}: Var, table: EscapeTable, current_depth: u32) -> (Var, EscapeTable) {
    match kind {
        VarKind::Simple(symbol) => match table.get(&symbol) {
            Some((table_depth, ..)) => {
                let mut new_table = table.clone();
                if current_depth > *table_depth {
                    new_table.insert(symbol.clone(), (*table_depth, true));
                }
                (Var {
                    kind: VarKind::Simple(symbol),
                    typ,
                    pos
                }, new_table)
            }
            None => panic!("Var {} does not exist!", symbol)
        },
        VarKind::Subscript(array, subscript) => {
            let (r_array, array_table) = trav_var(*array, table, current_depth);
            let (r_subscript, subscript_table) = trav_exp(*subscript, array_table, current_depth);
            (Var {
                kind: VarKind::Subscript(Box::new(r_array), Box::new(r_subscript)),
                typ,
                pos
            }, subscript_table)
        },
        VarKind::Field(record, field) => {
            let (r_record, record_table) = trav_var(*record, table, current_depth);
            (Var {
                kind: VarKind::Field(Box::new(r_record), field),
                typ,
                pos
            }, record_table)
        }
    }
}
fn trav_decs(mut decs: Vec<Dec>, table: EscapeTable, current_depth: u32) -> (Vec<Dec>, EscapeTable, EscapeTable) {
    // trav_decs returns:
    //  - the rebuilt declarations, with it's components recursively traversed.
    //  - a table with the escapes of outer variables
    //  - a table with the outer and inner variables
    let maybe_dec = decs.pop();
    if let Some(dec) = maybe_dec {
        match dec {
            Dec::VarDec(_VarDec{name, typ, init, ..}, pos) => {
                // traverse the init
                let (r_init, init_table) = trav_exp(*init.clone(), table, current_depth);
                // Add this dec to an inner table
                let mut inner_table = init_table.clone();
                inner_table.insert(name.clone(), (current_depth, false));
                // traverse previous declarations using the inner table
                let (mut r_later_decs, later_decs_outer_table, mut later_decs_inner_table) = trav_decs(decs, inner_table, current_depth);
                // find the resulting escape for this var, build a (so far) correct VarDec
                let escape = later_decs_inner_table.remove(&name).unwrap();
                let r_dec = Dec::VarDec(_VarDec{name: name.clone(), typ, init: Box::new(r_init), escape: escape.1}, pos);
                r_later_decs.push(r_dec);
                // Add the dec to the outer table, so that it can be escaped in the body.
                // If dec is repeated in we
                later_decs_inner_table.insert(name, escape);
                (r_later_decs, later_decs_outer_table, later_decs_inner_table)
            },
            Dec::FunctionDec(funtion_decs) => {
                let (r_function_decs, function_decs_table) = funtion_decs
                    .iter()
                    .fold((vec![], table.clone()), |(mut prev_decs, prev_table), (_FunctionDec{name, params, result, body}, pos)| {
                        let mut new_table = prev_table.clone();
                        for Field {name, ..} in params {
                            new_table.insert(name.clone(), (current_depth + 1, false));
                        }
                        let (r_body, mut body_table) = trav_exp(*body.clone(), new_table, current_depth + 1);
                        let mut r_params = vec![];
                        for Field {name, typ, ..} in params {
                            let escape = body_table.remove(name).unwrap().1;
                            r_params.push(Field{name: name.clone(), typ: typ.clone(), escape});
                        }
                        prev_decs.push((_FunctionDec{name: name.clone(), params: r_params, result: result.clone(), body: Box::new(r_body)}, *pos));
                        (prev_decs, merge_tables(prev_table, body_table))
                    });
                let (mut r_decs, outer_table, inner_table) = trav_decs(decs, table, current_depth);
                r_decs.push(Dec::FunctionDec(r_function_decs));
                (r_decs, merge_tables(outer_table, function_decs_table.clone()), merge_tables(inner_table, function_decs_table))
            },
            Dec::TypeDec(td) => {
                let (mut r_prev_decs, outer_table, inner_table) = trav_decs(decs, table.clone(), current_depth);
                r_prev_decs.push(Dec::TypeDec(td));
                (r_prev_decs, outer_table, inner_table)
            },
        }
    } else {
        (vec![], table.clone(), table)
    }
}

fn post_decs(decs: Vec<Dec>, table: EscapeTable) -> (Vec<Dec>, EscapeTable) {
    // Declarations have allready been traversed.
    // We need to set escapes and clean the table.
    fn post_decs_internal(mut decs: Vec<Dec>, table: EscapeTable, mut prev: Vec<Dec>) -> (Vec<Dec>, EscapeTable) {
        let maybe_dec = decs.pop();
        match maybe_dec {
            Some(dec) => {
                match dec {
                    Dec::VarDec(_VarDec{name, init, typ, ..}, pos) => {
                        // We don't remove to not break test37
                        let escape = table.get(&name).expect("post_decs").1;
                        prev.push(Dec::VarDec(_VarDec{name, init, typ, escape}, pos))
                    },
                    Dec::FunctionDec(fd) => {
                        prev.push(Dec::FunctionDec(fd));
                    },
                    Dec::TypeDec(td) => {
                        prev.push(Dec::TypeDec(td));
                    },
                }
                post_decs_internal(decs, table, prev)
            }
            None => (prev, table),
        }
    }
    let (decs, table) = post_decs_internal(decs, table, vec![]);
    (decs, table)
}
fn merge_tables(outer_table: EscapeTable, inner_table: EscapeTable) -> EscapeTable {
    outer_table
        .iter()
        .map(|(var, outer_escape)| {
            match inner_table.get(var) {
                Some(inner_escape) => (var.clone(), *inner_escape),
                None => (var.clone(), *outer_escape) // This here discards and clones the same value. There should be a better way.
            }
        })
        .collect()
}

fn trav_exp(AST {node, typ, pos}: AST, table: EscapeTable, current_depth: u32) -> (AST, EscapeTable) {
    // This function consumes consumes an AST and generates a new one with correct variable escapes.
    // If a variable is declared, then a new entry is inserted in the table with a False value (replacing if we are hiding a previous variable).
    //      Then, the lower branches are computed and the resulting table is checked for escapes.
    // If a variable is called, then the escape will be checked and set to true in the returned table if needed.
    // Function declarations have + 1 depth (a new frame is created)
    // All functions here should keep the invariant: the returned table only contains variables defined higher up in the AST, never in lower branches.
    // This means that a node that has a variable declaration should add, check and remove. Or clone or whatever.
    //
    // Branches are checked sequentially. This could be parallelized, but a table-combining function should be defined fot that.
    match node {
        Exp::Array {init, typ: array_type, size} => {
            let (r_init, r_table) = trav_exp(*init, table, current_depth);
            (AST {
                node: Exp::Array {init: Box::new(r_init), typ: array_type, size},
                typ,
                pos
            }, r_table)
        },
        Exp::Assign{var, exp} => {
            let (r_var, r_var_table) = trav_var(var, table, current_depth);
            let (r_exp, r_exp_table) = trav_exp(*exp, r_var_table, current_depth);
            (AST {
                node: Exp::Assign {var: r_var, exp: Box::new(r_exp)},
                typ,
                pos
            }, r_exp_table)
        },
        Exp::Call{func, args} => {
            let (r_args, r_table) : (Vec<AST>, EscapeTable)  = args
                .iter()
                .fold((vec![], table), move |(mut prev, table), exp| {
                    let (e, t) = trav_exp(exp.clone(), table, current_depth);  // move problems here?
                    prev.push(e);
                    (prev, t)
                });
            (AST {
                node: Exp::Call{func, args: r_args},
                pos,
                typ,
            }, r_table)
        },
        Exp::For{var, lo, hi, body, ..} => {
            // ForAST is kinda tricky. Variables referenced in range are outside, not the iterator.
            let (lo_exp, lo_table) = trav_exp(*lo, table, current_depth);
            let (hi_exp, hi_table) = trav_exp(*hi, lo_table, current_depth);
            let mut inner_table = hi_table.clone();
            inner_table.insert(var.clone(), (current_depth, false));
            let (body_exp, mut body_table) = trav_exp(*body, inner_table, current_depth);
            let escape = body_table.remove(&var).unwrap().1;
            (AST {
                node: Exp::For {var, lo: Box::new(lo_exp), hi: Box::new(hi_exp), body: Box::new(body_exp), escape},
                pos,
                typ,
            }, merge_tables(hi_table, body_table))
        },
        Exp::If{test, then_, else_} => {
            let (test_exp, test_table) = trav_exp(*test, table, current_depth);
            let (then_exp, then_table) = trav_exp(*then_, test_table, current_depth);
            if let Some(some_else) = else_ {
                let (else_exp, else_table) = trav_exp(*some_else, then_table, current_depth);
                (AST {
                    node: Exp::If{test: Box::new(test_exp), then_: Box::new(then_exp), else_: Some(Box::new(else_exp))},
                    pos,
                    typ,
                }, else_table)
            } else {
                (AST {
                    node: Exp::If{test: Box::new(test_exp), then_: Box::new(then_exp), else_: None},
                    pos,
                    typ,
                }, then_table)
            }
        },
        Exp::Let{mut decs, body} => {
            decs.reverse();
            let (r_decs, outer_table, decs_table) = trav_decs(decs, table.clone(), current_depth);
            let (r_body, body_table) = trav_exp(*body, decs_table, current_depth);
            let (rr_decs, post_body_table) = post_decs(r_decs, body_table);
            (AST {
                node: Exp::Let{decs: rr_decs, body: Box::new(r_body)},
                pos,
                typ,
            }, merge_tables(table, merge_tables(outer_table, post_body_table))) // I think post_body_table is always empty, idk...
        },
        Exp::Op{left, right, oper} => {
            let (left_exp, left_table) = trav_exp(*left, table, current_depth);
            let (right_exp, right_table) = trav_exp(*right, left_table, current_depth);
            (AST {
                node: Exp::Op{oper, left: Box::new(left_exp), right: Box::new(right_exp)},
                pos,
                typ,
            }, right_table)
        },
        Exp::Record{fields, typ: record_typ} => {
            let (r_fields, r_table) : (Vec<(Symbol, Box<AST>)>, EscapeTable)  = fields
                .iter()
                .fold((vec![], table), |(mut prev, table), (s, exp)| {
                    let (e, t) = trav_exp(*exp.clone(), table, current_depth);  // move problems here?
                    prev.push((s.clone(), Box::new(e)));
                    (prev, t)
                });
            (AST {
                node: Exp::Record{fields: r_fields, typ: record_typ},
                pos,
                typ,
            }, r_table)
        },
        Exp::Seq(exps) => {
            let (r_exps, r_table) : (Vec<AST>, EscapeTable)  = exps
                .iter()
                .fold((vec![], table), |(mut prev, table), exp| {
                    let (e, t) = trav_exp(exp.clone(), table, current_depth);  // move problems here?
                    prev.push(e);
                    (prev, t)
                });
            (AST {
                node: Exp::Seq(r_exps),
                pos,
                typ,
            }, r_table)
        },
        Exp::Var(var) => {
            let (r_var, r_table) = trav_var(var, table, current_depth);
            (AST {
                node: Exp::Var(r_var),
                pos,
                typ,
            }, r_table)
        },
        Exp::While{test, body} => {
            let (test_exp, test_table) = trav_exp(*test, table, current_depth);
            let (body_exp, body_table) = trav_exp(*body, test_table, current_depth);
            (AST {
                node: Exp::While{test: Box::new(test_exp), body: Box::new(body_exp)},
                pos,
                typ,
            }, body_table)

        },
        e => (
            AST {
                node: e,
                pos,
                typ,
            }, table
        ),
    }
}

pub fn find_escapes(exp: AST) -> AST {
    // Lo hacemos despues del tipado para que no salten aca errores de variables no declaradas.
    trav_exp(exp, EscapeTable::new(), 0).0
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;

    #[test]
    #[wasm_bindgen_test]
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
    #[wasm_bindgen_test]
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
    #[wasm_bindgen_test]
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
    #[wasm_bindgen_test]
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
    #[wasm_bindgen_test]
    fn escaped_for() {
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
    #[wasm_bindgen_test]
    fn not_escaped_for() {
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

}