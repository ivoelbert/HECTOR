use super::*;
use crate::utils::log;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment)
-> Result<AST, TypeError> {
    match node {
        Exp::Op{left, right, oper} => {
            let left_ast = type_exp(*left, type_env, value_env)?;
            let right_ast = type_exp(*right, type_env, value_env)?;
            let left_type = tipo_real(left_ast.typ.clone(), type_env);
            let right_type = tipo_real(right_ast.typ.clone(), type_env);
            let op_ast = AST {
                node: Exp::Op {
                    oper,
                    left: Box::new(left_ast),
                    right: Box::new(right_ast),
                },
                pos,
                typ: Arc::new(TigerType::TInt(R::RW))
            };
            match oper {
                Oper::EqOp | Oper::NeqOp => {
                    if left_type == right_type && *left_type != TigerType::TNil && *right_type != TigerType::TNil {
                        Ok(op_ast)
                    } else {
                        console_log!("op eq mismatch");
                        Err(TypeError::TypeMismatch(pos))
                    }
                },
                Oper::PlusOp | Oper::MinusOp | Oper::TimesOp | Oper::DivideOp => {
                    if es_int(&left_type) && es_int(&right_type)  {
                        Ok(op_ast)
                    } else {
                        console_log!("op plus mismatch");
                        Err(TypeError::TypeMismatch(pos))
                    }
                },
                Oper::LtOp | Oper::LeOp | Oper::GtOp | Oper::GeOp => {
                    if (es_int(&left_type) && es_int(&right_type)) || (*left_type == TigerType::TString && *right_type == TigerType::TString) {
                        Ok(op_ast)
                    } else {
                        console_log!("op lt mismatch");
                        Err(TypeError::TypeMismatch(pos))
                    }
                }
            }
        }
        _ => panic!("delegation errror on opexp::tipar")
    }
}