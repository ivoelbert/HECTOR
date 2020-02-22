use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    match node {
        Exp::Call {func: function_symbol, args,} => {
            let (formals, return_type) = match value_env.get(&function_symbol) {
                Some(EnvEntry::Func {
                    formals, result, ..
                }) => (formals, result),
                Some(EnvEntry::Var { .. }) => return Err(TypeError::NotFunctionVar(pos)),
                None => return Err(TypeError::UndeclaredFunction(pos)),
            };
            if formals.len() > args.len() {
                return Err(TypeError::TooFewArguments(pos));
            }
            if formals.len() < args.len() {
                return Err(TypeError::TooManyArguments(pos));
            }
            let typed_args : Vec<AST> = args
                .into_iter() // into_iter moves the values instead of borrowing
                .map(|arg| -> Result<AST, TypeError> {
                    type_exp(arg, type_env, value_env)
                })
                .collect::<Result<Vec<AST>, TypeError>>()?;
            // If any argument is not of it's formal type, fail.
            if formals.iter()
                .zip(typed_args.iter())
                .any(|(formal, typed_arg)| -> bool {
                    **formal != *typed_arg.typ
                }){
                return Err(TypeError::InvalidCallArgument(pos));
            };
            Ok(AST {
                node: Exp::Call {
                    func: function_symbol,
                    args: typed_args
                },
                pos,
                typ: return_type.clone()
            })
        }
        _ => panic!("delegation error"),
    }
}
