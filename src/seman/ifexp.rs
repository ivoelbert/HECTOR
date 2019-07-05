use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    // Tipar las 3 subexpresiones
    // Si alguna falla, propagar error.
    // Si el test no es Int, fallar.
    // Si then_ y else_ no tienen el mismo tipo, fallar.
    // Sino, devolver el tipo de then_ y else_
    // El ML lo hace mal. Devuelve siempre TUnit. Eso debe traer problemas despues.
    match exp {
        Exp {node: _exp, pos: _} => match _exp {
            _Exp::IfExp{test, then_, else_} => {
                let tytest = tipar_exp(test, type_env, value_env);
                let tythen = tipar_exp(then_, type_env, value_env);
                let tyelse = tipar_exp(else_, type_env, value_env);
                if let Ok(foo) = foo {
                    foo.
                }
            }
            _ => panic!("Mala delegacion en seman")
        }
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}