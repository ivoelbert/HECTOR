use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(
    exp: &Exp,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<Arc<TigerType>, TypeError> {
    let tipar_args = |args: &Vec<Exp>| -> Vec<Result<Arc<TigerType>, TypeError>> {
        args.iter()
            .map(|arg| type_exp(&*arg, type_env, value_env))
            .rev()
            .collect()
    };
    match exp {
        Exp {
            node:
                _Exp::Call {
                    func: function_symbol,
                    args,
                },
            pos,
        } => {
            let (formals, return_type) = match value_env.get(function_symbol) {
                Some(EnvEntry::Func {
                    formals, result, ..
                }) => (formals, result),
                Some(EnvEntry::Var { .. }) => return Err(TypeError::NotFunctionVar(*pos)),
                None => return Err(TypeError::UndeclaredFunction(*pos)),
            };
            if formals.len() > args.len() {
                return Err(TypeError::TooFewArguments(*pos));
            }
            if formals.len() < args.len() {
                return Err(TypeError::TooManyArguments(*pos));
            }
            for (arg_result, formal_type) in tipar_args(args).into_iter().zip(formals) {
                match &*(arg_result.clone()?) {
                    TigerType::TUnit => return Err(TypeError::InvalidCallArgument(*pos)),
                    _ => {
                        if tipo_real(arg_result?, type_env) != *formal_type {
                            return Err(TypeError::TypeMismatch(*pos));
                        }
                    }
                }
            }

            Ok((*return_type).clone())
        }
        _ => panic!("delegation error"),
    }
}
