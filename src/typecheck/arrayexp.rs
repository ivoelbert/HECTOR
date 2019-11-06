use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(
    exp: &Exp,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<Arc<TigerType>, TypeError> {
    // Buscar el tipo del array en el type_env
    // Si el tipo no existe, fallar.
    // Si el tipo existe pero no es un array, fallar.
    // Tipar el size. Si no es int, fallar.
    // Tipar el init. Si no es del mismo tipo del array, fallar.
    // Devolver TArray del tipo que encontramos en la tabla.
    use TigerType::*;
    match exp {
        Exp {
            node:
                _Exp::Array {
                    typ: array_of_symbol,
                    size: size_exp,
                    init: init_exp,
                },
            pos,
        } => match type_env.get(array_of_symbol) {
            Some(tiger_type) => match &**tiger_type {
                TArray(array_of_type, type_id) => match *type_exp(size_exp, type_env, value_env)? {
                    TInt(_) => {
                        let init_type = type_exp(init_exp, type_env, value_env)?;
                        if **array_of_type == *init_type {
                            Ok(Arc::new(TArray(array_of_type.clone(), *type_id)))
                        } else {
                            Err(TypeError::TypeMismatch(*pos))
                        }
                    }
                    _ => Err(TypeError::NonIntegerSize(*pos)),
                },
                _ => Err(TypeError::NotArrayType(*pos)),
            },
            None => Err(TypeError::UndeclaredType(*pos)),
        },
        _ => panic!("le llego algo nada que ver a arrayexp::tipar"),
    }
}