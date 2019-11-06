use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Arc<TigerType>, TypeError> {
    use TigerType::*;
    use super::varexp::typecheck_var;
    match exp {
        Exp {node: _Exp::Assign{var , exp: value_exp}, pos} => {
            let var_type = match &*typecheck_var(var, *pos, type_env, value_env)? {
                TInt(R::RO) => return Err(TypeError::ReadOnlyAssignment(*pos)),
                tiger_type => tiger_type.clone(),
            };
            let value_type = &*type_exp(value_exp, type_env, value_env)?;
            if var_type == *value_type {
                Ok(Arc::new(TUnit))
            }
            else {
                Err(TypeError::TypeMismatch(*pos))
            }
        },
        _ => panic!("Mala delegacion en seman")
    }
}