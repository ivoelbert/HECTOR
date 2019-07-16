use std::result::Result;

use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    use Var::*;
    use super::varexp::tipar_var;
    match exp {
        Exp {node: _Exp::AssignExp{var , exp: value_exp}, pos} => {
            let var_type = match tipar_var(var, pos, type_env.clone(), value_env.clone()) {
                Ok(TInt(R::RO)) => return Err(TypeError::ReadOnlyAssignment(pos)),
                Ok(tipo) => tipo,
                Err(type_error) => return Err(type_error)
            };
            let value_type = match tipar_exp(*value_exp, type_env.clone(), value_env.clone()) {
                Ok(tipo) => tipo,
                Err(type_error) => return Err(type_error)
            };
            if var_type == value_type {
                return Ok(TUnit);
            }
            else {
                return Err(TypeError::TypeMismatch(pos));
            }
        },
        _ => panic!("Mala delegacion en seman")
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}