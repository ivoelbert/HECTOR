use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env:& ValueEnviroment
) -> Result<AST, TypeError> {
    match node {
        Exp::For {var, lo, hi, body, escape} => {
            let lo_ast = type_exp(*lo, type_env, value_env)?;
            let hi_ast = type_exp(*hi, type_env, value_env)?;
            let lo_type = tipo_real(lo_ast.typ.clone(), type_env);
            let hi_type = tipo_real(hi_ast.typ.clone(), type_env);
            if !es_int(&lo_type) || !es_int(&hi_type) {
                return Err(TypeError::NonIntegerForRange(pos));
            }
            let mut new_value_env = value_env.clone();
            new_value_env.insert(var.clone(), EnvEntry::Var {
                ty: Arc::new(TigerType::TInt(R::RO)),
            });
            let body_ast = type_exp(*body, type_env, &new_value_env)?;
            if *body_ast.typ != TigerType::TUnit {
                return Err(TypeError::NonUnitBody(pos));
            }
            Ok(AST {
                node: Exp::For {
                    var,
                    lo: Box::new(lo_ast),
                    hi: Box::new(hi_ast),
                    body: Box::new(body_ast),
                    escape
                },
                pos,
                typ: Arc::new(TigerType::TUnit)
            })
        }
        _ => panic!("delegation panic in forexp::tipar")
    }
}
