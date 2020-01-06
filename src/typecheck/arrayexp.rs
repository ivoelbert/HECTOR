use crate::typecheck::*;

pub fn typecheck(
    AST {node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    // Get array type in value_env. If it's not an array, fail
    // Type the size. If it's not int, fail
    // Type the init. If it's not the same as the array, fail
    // Return TArray of the type found in hte env
    use TigerType::*;
    match node {
        Exp::Array {typ: array_of_symbol, size: size_exp, init: init_exp, } => {
            let array_type = match type_env.get(&array_of_symbol) {
                Some(tiger_type) => &**tiger_type,
                None => return Err(TypeError::UndeclaredType(pos)),
            };
            match array_type {
                TArray(array_of_type, type_id) => {
                    let size_ast = type_exp(*size_exp, type_env, value_env)?;
                    match *size_ast.typ {
                        TInt(_) => {
                            let init_ast = type_exp(*init_exp, type_env, value_env)?;
                            if **array_of_type == *init_ast.typ {
                                Ok(AST{
                                    node: Exp::Array {
                                        size: Box::new(size_ast),
                                        init: Box::new(init_ast),
                                        typ: array_of_symbol
                                    },
                                    pos,
                                    typ: Arc::new(TArray(array_of_type.clone(), type_id.clone()))})
                            } else {
                                Err(TypeError::TypeMismatch(pos))
                            }
                        }
                        _ => Err(TypeError::NonIntegerSize(pos)),
                    }
                },
                _ => Err(TypeError::NotArrayType(pos)),
            }
        }
        _ => panic!("le llego algo nada que ver a arrayexp::tipar"),
    }
}
