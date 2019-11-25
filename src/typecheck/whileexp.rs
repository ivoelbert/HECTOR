use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    match node {
        Exp::While {test, body} => {
            let test_ast = type_exp(*test, type_env, value_env)?;
            if !es_int(&tipo_real(test_ast.typ.clone(), type_env)) {
                return Err(TypeError::NonIntegerCondition(pos));
            }
            let body_ast = type_exp(*body, type_env, value_env)?;
            match *body_ast.typ {
                TigerType::TUnit => Ok(AST {
                    node: Exp::While{
                        test: Box::new(test_ast),
                        body: Box::new(body_ast)
                    },
                    typ: Arc::new(TigerType::TUnit),
                    pos
                }),
                _ => Err(TypeError::NonUnitBody(pos))
            }
        }
        _ => panic!("le llego cualquier cosa a whileexp::tipar")
    }
}

