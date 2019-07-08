use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar<'a>(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo<'a>, TypeError> {
    // Buscar el tipo del array en el type_env
    // Si el tipo no existe, fallar.
    // Si el tipo existe pero no es un array, fallar.
    // Tipar el size. Si no es int, fallar.
    // Tipar el init. Si no es del mismo tipo del array, fallar.
    // Devolver TArray del tipo que encontramos en la tabla.
    return Ok(Tipo::TUnit);
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}