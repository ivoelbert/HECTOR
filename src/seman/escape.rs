use std::collections::HashMap;
use super::super::ast::tigerabs::*;

type EscapeTable = HashMap<Symbol, (u32, bool)>;

fn trav_var(var: Var, mut table: EscapeTable, current_depth: u32) -> (Var, EscapeTable) {
    match var {
        Var::SimpleVar(symbol) => match table.get(&symbol) {
            Some((table_depth, escape)) => {
                if current_depth > *table_depth {
                    table.insert(symbol.clone(), (*table_depth, true));
                }
                (Var::SimpleVar(symbol), table)
            }
            None => panic!("Var {} does not exist!", symbol)
        },
        Var::SubscriptVar(array, subscript) => {
            let (r_array, array_table) = trav_var(*array, table, current_depth);
            let (r_subscript, subscript_table) = trav_exp(*subscript, array_table, current_depth);
            (Var::SubscriptVar(Box::new(r_array), Box::new(r_subscript)), subscript_table)
        },
        Var::FieldVar(record, field) => {
            let (r_record, record_table) = trav_var(*record, table, current_depth);
            (Var::FieldVar(Box::new(r_record), field), record_table)
        }
    }
}
fn trav_decs(decs: Vec<Dec>, mut table: EscapeTable, current_depth: u32) -> (Vec<Dec>, EscapeTable, EscapeTable) {
    // trav_decs returns:
    //  - the rebuilt declarations, with it's components recursibely traversed.
    //  - a table with the escapes of outer variables
    //  - a table with the outer and inner variables
    let maybe_dec = decs.pop();
    match maybe_dec {
        Some(dec) => {
            match dec {
                Dec::VarDec(_VarDec{name, typ, init, ..}, pos) => {
                    let (r_init, outer_table) = trav_exp(*init, table, current_depth);
                    let inner_table = table.clone();
                    inner_table.insert(name, (current_depth, false));
                    let (r_decs, decs_outer_table, decs_table) = trav_decs(decs, inner_table, current_depth);
                    let escape = decs_outer_table.remove(&name).unwrap();
                    let r_dec = Dec::VarDec(_VarDec{name, typ, init: Box::new(r_init), escape: escape.1}, pos);
                    r_decs.push(r_dec);
                    decs_table.insert(name, escape);
                    (r_decs, merge_tables(outer_table, decs_outer_table), decs_table)

                },
                Dec::FunctionDec(fd) => {
                    (vec![Dec::FunctionDec(fd)], table, table.clone())
                }, // TODO: depth + here
                Dec::TypeDec(td) => {
                    (vec![Dec::TypeDec(td)], table, table.clone())
                },
            }
        }
        None => (vec![], table, table.clone()),
    }
}

fn post_decs(decs: Vec<Dec>, mut table: EscapeTable) -> (Vec<Dec>, EscapeTable) {
    // Declarations have allready been traversed.
    // We need to set escapes and clean the table.
    fn post_decs_internal(decs: Vec<Dec>, table: EscapeTable, prev: Vec<Dec>) -> (Vec<Dec>, EscapeTable) {
        let maybe_dec = decs.pop();
        match maybe_dec {
            Some(dec) => {
                match dec {
                    Dec::VarDec(_VarDec{name, init, typ, ..}, pos) => {
                        let escape = table.remove(&name).unwrap().1;
                        prev.push(Dec::VarDec(_VarDec{name, init, typ, escape}, pos))
                    },
                    Dec::FunctionDec(fd) => {
                        // LAST TO DO
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
    decs.reverse();
    (decs, table)
}
fn merge_tables(mut outer_table: EscapeTable, inner_table: EscapeTable) -> EscapeTable {
    outer_table
        .iter()
        .map(|(var, outer_escape)| {
            match inner_table.get(var) {
                Some(inner_escape) => (var.clone(), *inner_escape),
                None => (var.clone(), *outer_escape)
            }
        })
        .collect()
}

fn trav_exp(Exp {node, pos}: Exp, mut table: EscapeTable, current_depth: u32) -> (Exp, EscapeTable) {
    // This function consumes consumes an Exp and generates a new one with correct variable escapes.
    // If a variable is declared, then a new entry is inserted in the table with a false value (replacing if necesary).
    //      Then, the lower branches are computed and the resulting table is checked for escapes.
    // If a variable is called, then the escape will be checked and set to true in the returned table if needed.
    // Function declarations have + 1 depth (a new frame is created)
    // All functions here should keep the invariant: the returned table only contains variables defined higher up in the AST, never in lower branches.
    // This means that a node that has a variable declaration should add, check and remove. Or clone or whatever.
    //
    // Branches are checked sequentially. This could be parallelized, but a table-combining function should be defined fot that.
    match node {
        _Exp::ArrayExp {init, typ, size} => {
            let (r_init, r_table) = trav_exp(*init, table, current_depth);
            (Exp {
                node: _Exp::ArrayExp {init: Box::new(r_init), typ, size},
                pos
            }, r_table)
        },
        _Exp::AssignExp{var, exp} => {
            let (r_var, r_var_table) = trav_var(var, table, current_depth);
            let (r_exp, r_exp_table) = trav_exp(*exp, r_var_table, current_depth);
            (Exp {
                node: _Exp::AssignExp {var: r_var, exp: Box::new(r_exp)},
                pos
            }, r_exp_table)
        },
        _Exp::CallExp{func, args} => {
            let (r_args, r_table) : (Vec<Exp>, EscapeTable)  = args
                .iter()
                .fold((vec![], table), |(prev, table), exp| {
                    let (e, t) = trav_exp(*exp, table, current_depth);  // move problems here?
                    prev.push(e);
                    (prev, t)
                });
            (Exp {
                node: _Exp::CallExp{func, args: r_args},
                pos
            }, r_table)
        },
        _Exp::ForExp{var, lo, hi, body, ..} => {
            // ForExp is kinda tricky. Variables referenced in range are outside, not the iterator.
            let (lo_exp, lo_table) = trav_exp(*lo, table, current_depth);
            let (hi_exp, hi_table) = trav_exp(*hi, lo_table, current_depth);
            let inner_table = hi_table.clone();
            inner_table.insert(var.clone(), (current_depth, false));
            let (body_exp, body_table) = trav_exp(*body, inner_table, current_depth);
            let escape = body_table.remove(&var).unwrap().1;
            (Exp {
                node: _Exp::ForExp {var, lo: Box::new(lo_exp), hi: Box::new(hi_exp), body: Box::new(body_exp), escape},
                pos
            }, merge_tables(hi_table, inner_table))
        },
        _Exp::IfExp{test, then_, else_} => {
            let (test_exp, test_table) = trav_exp(*test, table, current_depth);
            let (then_exp, then_table) = trav_exp(*then_, test_table, current_depth);
            if let Some(some_else) = else_ {
                let (else_exp, else_table) = trav_exp(*some_else, then_table, current_depth);
                (Exp {
                    node: _Exp::IfExp{test: Box::new(test_exp), then_: Box::new(then_exp), else_: Some(Box::new(else_exp))},
                    pos
                }, else_table)
            } else {
                (Exp {
                    node: _Exp::IfExp{test: Box::new(test_exp), then_: Box::new(then_exp), else_: None},
                    pos
                }, then_table)
            }
        },
        _Exp::LetExp{decs, body} => {
            let (r_decs, outer_table, decs_table) = trav_decs(decs, table.clone(), current_depth);
            let (r_body, body_table) = trav_exp(*body, decs_table, current_depth);
            let (rr_decs, post_body_table) = post_decs(decs, body_table);
            (Exp {
                node: _Exp::LetExp{decs: rr_decs, body: Box::new(r_body)},
                pos
            }, merge_tables(table, merge_tables(outer_table, post_body_table))) // I think post_body_table is always empty, idk...
        },
        _Exp::OpExp{left, right, oper} => {
            let (left_exp, left_table) = trav_exp(*left, table, current_depth);
            let (right_exp, right_table) = trav_exp(*right, left_table, current_depth);
            (Exp {
                node: _Exp::OpExp{oper, left: Box::new(left_exp), right: Box::new(right_exp)},
                pos
            }, right_table)
        },
        _Exp::RecordExp{fields, typ} => {
            let (r_fields, r_table) : (Vec<(Symbol, Box<Exp>)>, EscapeTable)  = fields
                .iter()
                .fold((vec![], table), |(prev, table), (s, exp)| {
                    let (e, t) = trav_exp(**exp, table, current_depth);  // move problems here?
                    prev.push((s.clone(), Box::new(e)));
                    (prev, t)
                });
            (Exp {
                node: _Exp::RecordExp{fields: r_fields, typ},
                pos
            }, r_table)
        },
        _Exp::SeqExp(exps) => {
            let (r_exps, r_table) : (Vec<Exp>, EscapeTable)  = exps
                .iter()
                .fold((vec![], table), |(prev, table), exp| {
                    let (e, t) = trav_exp(*exp, table, current_depth);  // move problems here?
                    prev.push(e);
                    (prev, t)
                });
            (Exp {
                node: _Exp::SeqExp(r_exps),
                pos
            }, r_table)
        },
        _Exp::VarExp(var) => {
            let (r_var, r_table) = trav_var(var, table, current_depth);
            (Exp {
                node: _Exp::VarExp(r_var),
                pos
            }, r_table)
        },
        _Exp::WhileExp{test, body} => {
            let (test_exp, test_table) = trav_exp(*test, table, current_depth);
            let (body_exp, body_table) = trav_exp(*body, test_table, current_depth);
            (Exp {
                node: _Exp::WhileExp{test: Box::new(test_exp), body: Box::new(body_exp)},
                pos
            }, body_table)

        },
        e => (
            Exp {
                node: e,
                pos
            }, table
        ),
    }
}

pub fn find_escapes(exp: Exp) -> Exp {
    // Lo hacemos despues del tipado para que no salten aca errores de variables no declaradas.
    trav_exp(exp, EscapeTable::new(), 0).0
}