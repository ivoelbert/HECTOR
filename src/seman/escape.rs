use std::collections::HashMap;
use super::super::ast::tigerabs::*;

type EscapeTable<'a> = HashMap<Symbol, (u32, &'a mut bool)>;

fn trav_var<'a>(var: &'a mut Var<'a>, table: &'a mut EscapeTable<'a>, current_depth: u32) {
    match var {
        Var::SimpleVar(symbol) => match table.get_mut(symbol) {
            Some((table_depth, escape)) => {
                if current_depth > *table_depth {
                    **escape = true;
                }
            }
            None => panic!("Var {} does not exist!", symbol)
        },
        Var::SubscriptVar(array, subscript) => {
            trav_var(array, table, current_depth);
            trav_exp(subscript, table, current_depth);
        },
        Var::FieldVar(record, field) => {
            trav_var(record, table, current_depth);
        }
    }
}

fn trav_decs<'a>(decs: &mut [Dec<'a>], table: &'a mut EscapeTable<'a>, current_depth: u32) -> &'a mut EscapeTable<'a> {
    fn aux<'b>(dec: &'b mut Dec<'b>, table: &'b mut EscapeTable<'b>, current_depth: u32)  -> &'b mut EscapeTable<'b> {
        match dec {
            Dec::FunctionDec{..} => table,
            Dec::VarDec(_VarDec{name, escape, init, ..}, _) => {
                trav_exp(&mut *init, table, current_depth);
                table.insert(name.clone(), (current_depth, escape));
                table
            },
            Dec::TypeDec{..} => table,
        }
    }
    match decs.split_first_mut() {
        None => table,
        Some((dec, rest)) => {
            let new_table = table;
            trav_decs(rest, new_table, current_depth)
        }
    }
}

fn trav_exp<'a>(Exp {node, ..}: &'a mut Exp<'a>, table: &'a mut EscapeTable<'a>, current_depth: u32) {
    match node {
        _Exp::ArrayExp {init, ..} => {
            trav_exp(init, table, current_depth)
        },
        _Exp::AssignExp{var, exp} => {
            trav_var(var, table, current_depth);
            trav_exp(exp, table, current_depth)
        },
        _Exp::BreakExp => {
            ()
        },
        _Exp::CallExp{args, ..} => {
            trav_exp(&mut posed_exp(_Exp::SeqExp(*args), 0, 0), table, current_depth)
        },
        _Exp::ForExp{var, escape, lo, hi, body} => {
            trav_exp(&mut *lo, table, current_depth);
            trav_exp(&mut *hi, table, current_depth);
            table.insert(var.clone(), (current_depth, &mut escape));
            trav_exp(&mut *body, table, current_depth);
        },
        _Exp::IfExp{test, then_, else_} => {
            trav_exp(&mut *test, table, current_depth);
            trav_exp(&mut *then_, table, current_depth);
            if let Some(else_exp) = else_ {
                trav_exp(&mut *test, table, current_depth);
            }
        },
        _Exp::LetExp{decs, body} => {
            trav_exp(&mut *body, trav_decs(&mut *decs, table, current_depth), current_depth);
        },
        _Exp::OpExp{left, right, ..} => {
            trav_exp(&mut *left, table, current_depth);
            trav_exp(&mut *right, table, current_depth);
        },
        _Exp::RecordExp{fields, ..} => {
            fields
                .iter_mut()
                .map(|(_, exp)| trav_exp(exp, table, current_depth));
        },
        _Exp::SeqExp(exps) => {
            exps
                .iter_mut()
                .map(|exp| trav_exp(&mut *exp, table, current_depth));
        },
        _Exp::VarExp(var) => {
            trav_var(var, table, current_depth);
        },
        _Exp::WhileExp{test, body} => {
            trav_exp(&mut *test, table, current_depth);
            trav_exp(&mut *body, table, current_depth);
        },
        _Exp::IntExp(..)
        | _Exp::UnitExp
        | _Exp::StringExp(..)
        | _Exp::NilExp
        | _Exp::BreakExp => (),
    }
}

pub fn find_escapes<'a>(exp: &'a mut Exp<'a>) -> () {
    // Lo hacemos despues del tipado para que no salten aca errores de variables no declaradas.
    trav_exp(exp, &mut EscapeTable::new(), 0);
}