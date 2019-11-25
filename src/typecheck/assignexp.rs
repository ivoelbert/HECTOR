use super::*;

pub fn typecheck(
    ast: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    use TigerType::*;
    use super::varexp::typecheck_var;
    match ast {
        AST {node: Exp::Assign{var, exp: value_exp}, pos, ..} => {
            let typed_var = typecheck_var(var, type_env, value_env)?;
            let var_type = match &*typed_var.typ {
                TInt(R::RO) => return Err(TypeError::ReadOnlyAssignment(pos)),
                tiger_type => tiger_type.clone(),
            };
            let value_ast = type_exp(*value_exp, type_env, value_env)?;
            if var_type == *value_ast.typ {
                Ok(AST {
                    node: Exp::Assign{
                        var: typed_var,
                        exp: Box::new(value_ast)
                    },
                    pos,
                    typ: Arc::new(TUnit)
                })
            }
            else {
                Err(TypeError::TypeMismatch(pos))
            }
        },
        _ => panic!("Mala delegacion en seman")
    }
}