use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    // match exp {
    //     Exp {node: _exp, pos: _} =>
    //         match _exp {
    //             _Exp::AssignExp{var: SimpleVar(symbol), exp: Box(assigned_exp)} => {
    //                 // Buscar el simbolo en la tabla
    //                 // Si no existe, no es variable o es read-only, fallamos.
    //                 // Tipamos la expresion asignada.
    //                 // Si falla, propagamos el error.
    //                 // Si es Nil, fallamos.
    //                 // Si el tipo de la variable y de lo asignado son distintos, fallamos.
    //                 // Devolvemos TUnit.
    //                 return Ok(Tipo::TUnit);
    //             },
    //             _Exp::AssignExp{var, exp: Box(assigned_exp)} => {
    //                 // Tipamos la variable
    //                 // Tipamos la expresion
    //                 // Si son distintos fallamos.
    //                 // Devolvemos TUnit
    //                 return Ok(Tipo::TUnit);
    //             }
    //         },
    // }
    // panic!("Mala delegacion en seman");
    return Ok(Tipo::TUnit);
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}