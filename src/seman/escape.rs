use std::collections::HashMap;
use super::super::ast::tigerabs::*;

type EscapeTable = HashMap<Symbol, (u32, bool)>;

fn mix_tables(tables: Vec<EscapeTable>) -> EscapeTable {

}

fn trav_var(mut var: Var, mut table: EscapeTable, current_depth: u32) -> (Var, EscapeTable) {
    match var {
        Var::SimpleVar(symbol) => match table.get(&symbol) {
            Some((table_depth, escape)) => {
                if current_depth > *table_depth {
                    table.entry(symbol.clone()).or_insert((0, true));
                };
                (var, table)
            }
            None => panic!("Var {} does not exist!", symbol)
        },
        Var::SubscriptVar(array, subscript) => {
            let (_, table1) = trav_var(*array, table, current_depth);
            let (_, table2) = trav_exp(*subscript, table.clone(), current_depth);
            (var, mix_tables(vec![table1, table2]))
        },
        Var::FieldVar(record, field) => {
            let (_, new_table) = trav_var(*record, table, current_depth);
            (var, new_table)
        }
    }
}

fn trav_decs(mut dec: Dec, mut table: EscapeTable, current_depth: u32) -> (Dec, EscapeTable) {
    match dec {
        Dec::VarDec(_VarDec(name, escape, _, init)) => {
            let (_, table2) = trav_exp(*subscript, table.clone(), current_depth);
        }
    }
}

fn trav_exp(mut exp: Exp, mut table: EscapeTable, current_depth: u32) -> (Exp, EscapeTable) {

}

pub fn find_escapes(exp: Exp) -> Exp {
    // Lo hacemos despues del tipado para que no salten aca errores de variables no declaradas.
    trav_exp(exp, EscapeTable::new(), 0).0
}