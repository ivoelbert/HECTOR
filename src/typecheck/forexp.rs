use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env:& ValueEnviroment) -> Result<Arc<TigerType>, TypeError> {
    use TigerType::*;
    match exp { Exp {node: _Exp::For {var, lo, hi, body, ..}, pos} => {
        let lo_type = tipo_real(type_exp(&*lo, type_env, value_env)?, type_env);
        let hi_type = tipo_real(type_exp(&*hi, type_env, value_env)?, type_env);
        if !es_int(&lo_type) || !es_int(&hi_type) {
            return Err(TypeError::NonIntegerForRange(*pos));
        }
        let mut new_value_env = value_env.clone();
        new_value_env.insert(var.clone(), EnvEntry::Var {
            ty: Arc::new(TInt(R::RO)),
        });
        match *type_exp(&*body, type_env, &new_value_env)? {
            TUnit => (),
            _ => return Err(TypeError::NonUnitBody(*pos))
        };
        Ok(Arc::new(TUnit))
    }
    _ => panic!("delegation panic in forexp::tipar")
    }
}
