use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Arc<TigerType>, TypeError> {
    // TODO this excludes string operations, fml
    match exp { Exp {node: _Exp::Op{left, right, oper}, pos} => {
        let left_type = tipo_real(type_exp(&*left, type_env, value_env)?, type_env);
        let right_type = tipo_real(type_exp(&*right, type_env, value_env)?, type_env);
        match oper {
            Oper::EqOp | Oper::NeqOp => {
                if left_type == right_type && *left_type != TigerType::TNil && *right_type != TigerType::TNil {
                    Ok(Arc::new(TigerType::TInt(R::RW)))
                } else {
                    Err(TypeError::TypeMismatch(*pos))
                }
            },
            Oper::PlusOp | Oper::MinusOp | Oper::TimesOp | Oper::DivideOp => {
                if es_int(&left_type) && es_int(&right_type)  {
                    Ok(Arc::new(TigerType::TInt(R::RW)))
                } else {
                    Err(TypeError::TypeMismatch(*pos))
                }
            },
            Oper::LtOp | Oper::LeOp | Oper::GtOp | Oper::GeOp => {
                if es_int(&left_type) && es_int(&right_type)  {
                    Ok(Arc::new(TigerType::TInt(R::RW)))
                } else if *left_type == TigerType::TString && *right_type == TigerType::TString {
                    Ok(Arc::new(TigerType::TString))
                } else {
                    Err(TypeError::TypeMismatch(*pos))
                }
            }
        }
    }
    _ => panic!("delegation errror on opexp::tipar")
    }
}