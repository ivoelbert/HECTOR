use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    // Buscar el tipo del array en el type_env
    // Si el tipo no existe, fallar.
    // Si el tipo existe pero no es un array, fallar.
    // Tipar el size. Si no es int, fallar.
    // Tipar el init. Si no es del mismo tipo del array, fallar.
    // Devolver TArray del tipo que encontramos en la tabla.
    // let pos = exp.pos;
    // match exp.node {
    //     _Exp::ArrayExp{typ, size, init} => {
    //         let size_type = tipar_exp(Box::leak(size), type_env, value_env);
    //         let size_type = 
    //     },
    //     _ => panic!("mal delegado")
    // }
    use _Exp::*;
    use Var::*;
    use Tipo::*;
    match exp {
        Exp { node: ArrayExp { typ: array_of_symbol, size: size_exp, init: init_exp}, pos}
            => match type_env.get(&array_of_symbol) {
                Some(TArray(array_of_type, type_id)) => match tipar_exp(*size_exp, type_env.clone(), value_env.clone()) {
                    Ok(TInt(_)) => match tipar_exp(*init_exp, type_env.clone(), value_env.clone()) {
                        Ok(init_type) => if **array_of_type == init_type {
                            Ok(TArray(Box::new(*array_of_type.clone()), *type_id))
                        } else {
                            Err(TypeError::TypeMismatch(pos))
                        },
                        Err(e) => Err(e)
                    },
                    Ok(_) => Err(TypeError::NonIntegerSize(pos)),
                    Err(e) => Err(e)
                },
                Some(_) => Err(TypeError::NotArrayType(pos)),
                None => Err(TypeError::UndeclaredType(pos))
            },
        _ => panic!("le llego algo nada que ver a arrayexp::tipar")
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}