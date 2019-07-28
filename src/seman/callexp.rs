use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    let tipar_args = |args: Vec<Box<Exp>>| -> Vec<Result<Tipo, TypeError>> {
        return args.into_iter().map(|arg| tipar_exp(*arg, type_env.clone(), value_env.clone())).rev().collect();
    };
    match exp {
        Exp {node: _Exp::CallExp {func: function_symbol, args}, pos} => {
            let (formals, return_type) = match value_env.get(&function_symbol) {
                Some(EnvEntry::Func {label: _, formals, result, external: _}) => (formals, result),
                Some(EnvEntry::Var {ty: _, access: _, level: _}) => return Err(TypeError::NotFunctionVar(pos)),
                None => return Err(TypeError::UndeclaredFunction(pos))
            };
            if formals.len() > args.len() {
                return Err(TypeError::TooFewArguments(pos));
            }
            if formals.len() < args.len() {
                return Err(TypeError::TooManyArguments(pos))
            }
            for (arg_result, formal_type) in tipar_args(args).into_iter().zip(formals) {
                match arg_result {
                    Ok(Tipo::TUnit) => return Err(TypeError::InvalidCallArgument(pos)),
                    Err(type_error) => return Err(type_error),
                    // esto tedria que ser tipoReal(argtype)
                    Ok(arg_type) => if arg_type !=  *formal_type {return Err(TypeError::TypeMismatch(pos))},
                }
            }

            return Ok(return_type.clone());
        },
        _ => panic!("le llego cualquier cosa a callexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}