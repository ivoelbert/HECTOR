use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp { Exp {node: _Exp::OpExp{left, right, oper}, pos} => {
        let left_type = tipo_real(tipar_exp(&*left, type_env, value_env)?, type_env);
        let right_type = tipo_real(tipar_exp(&*right, type_env, value_env)?, type_env);
        match oper {
            Oper::EqOp | Oper::NeqOp => {
                if left_type == right_type && left_type != Tipo::TNil && right_type != Tipo::TNil {
                    Ok(Tipo::TInt(R::RW))
                } else {
                    Err(TypeError::TypeMismatch(*pos))
                }
            },
            Oper::PlusOp | Oper::MinusOp | Oper::TimesOp | Oper::DivideOp => {
                if es_int(&left_type) && es_int(&right_type)  {
                    Ok(Tipo::TInt(R::RW))
                } else {
                    Err(TypeError::TypeMismatch(*pos))
                }
            },
            Oper::LtOp | Oper::LeOp | Oper::GtOp | Oper::GeOp => {
                if es_int(&left_type) && es_int(&right_type)  {
                    Ok(Tipo::TInt(R::RW))
                } else if left_type == Tipo::TString && right_type == Tipo::TString {
                    Ok(Tipo::TString)
                } else {
                    Err(TypeError::TypeMismatch(*pos))
                }
            }
        }
    }
    _ => panic!("delegation errror on opexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}