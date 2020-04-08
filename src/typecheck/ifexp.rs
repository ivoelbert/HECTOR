use super::*;

use crate::utils::log;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    match node {
        Exp::If{test, then_, else_} => {
            let test_ast = type_exp(*test, type_env, value_env)?;
            if !es_int(&tipo_real(test_ast.typ.clone(), type_env)) {
                return Err(TypeError::NonIntegerCondition(pos));
            }
            let then_ast = type_exp(*then_, type_env, value_env)?;
            match else_ {
                Some(else_exp) => {
                    let else_ast = type_exp(*else_exp, type_env, value_env)?;
                    if else_ast.typ != then_ast.typ {
                        console_log!("if mismatch");
                        return Err(TypeError::ThenElseTypeMismatch(pos))
                    }
                    let typ = then_ast.typ.clone();
                    Ok(AST {
                        node: Exp::If {
                            test: Box::new(test_ast),
                            then_: Box::new(then_ast),
                            else_: Some(Box::new(else_ast)),
                        },
                        pos,
                        typ
                    })
                }
                None => if *then_ast.typ == TigerType::TUnit {
                    let typ = then_ast.typ.clone();
                    Ok(AST {
                        node: Exp::If {
                            test: Box::new(test_ast),
                            then_: Box::new(then_ast),
                            else_: None
                        },
                        pos,
                        typ
                    })
                } else {
                    Err(TypeError::NonUnitBody(pos))
                }
            }
        }
        _ => panic!("Delegation error on ifexp::tipar")
    }
}
