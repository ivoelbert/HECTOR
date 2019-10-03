use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    use super::varexp::typecheck_var;
    match exp {
        Exp {node: _Exp::AssignExp{var , exp: value_exp}, pos} => {
            let var_type = match typecheck_var(var, *pos, type_env, value_env) {
                Ok(TInt(R::RO)) => return Err(TypeError::ReadOnlyAssignment(*pos)),
                Ok(tipo) => tipo,
                Err(type_error) => return Err(type_error)
            };
            let value_type = match type_exp(value_exp, type_env, value_env) {
                Ok(tipo) => tipo,
                Err(type_error) => return Err(type_error)
            };
            if var_type == value_type {
                Ok(TUnit)
            }
            else {
                Err(TypeError::TypeMismatch(*pos))
            }
        },
        _ => panic!("Mala delegacion en seman")
    }
}