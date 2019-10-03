use crate::ast::*;
use crate::typecheck::*;
use crate::tree::Access;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env:& ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    match exp { Exp {node: _Exp::ForExp {var, lo, hi, body, ..}, pos} => {
        let lo_type = tipo_real(type_exp(&*lo, type_env, value_env)?, type_env);
        let hi_type = tipo_real(type_exp(&*hi, type_env, value_env)?, type_env);
        if !es_int(&lo_type) || !es_int(&hi_type) {
            return Err(TypeError::NonIntegerForRange(*pos));
        }
        let mut new_value_env = value_env.clone();
        new_value_env.insert(var.clone(), EnvEntry::Var {
            ty: TInt(R::RO),
            access: Access::InFrame(0),
            level: 0
        });
        match type_exp(&*body, type_env, &new_value_env) {
            Ok(TUnit) => (),
            Ok(_) => return Err(TypeError::NonUnitBody(*pos)),
            Err(type_error) => return Err(type_error)
        };
        Ok(TUnit)
    }
    _ => panic!("delegation panic in forexp::tipar")
    }
}
