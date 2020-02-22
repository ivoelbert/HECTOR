use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<AST, TypeError> {
    let type_fields = |args: Vec<(Symbol, Box<AST>)>| -> Result<HashMap<Symbol, AST>, TypeError> {
        args
            .into_iter()
            .map(|(symbol, ast)| -> Result<(Symbol, AST), TypeError> {
                Ok((symbol, type_exp(*ast, type_env, value_env)?))
            })
            .collect()
        //args.iter().map(|arg| (arg.0.clone(), type_exp(*arg.1, type_env, value_env))).rev().collect()
    };
    match node {
        Exp::Record{fields, typ: record_type_symbol} => {
            // Type the fields
            // If the record type is not a record type, error.
            // If some field doesn't match the formal type, error.
            // if field <-> formals is a 1:1, error.
            let mut typed_fields = type_fields(fields)?;
            let record_type = match type_env.get(&record_type_symbol) {
                Some(tipo) => tipo_real(tipo.clone(), type_env),
                None => {
                    console_log!("arrayexp undeclared");
                    return Err(TypeError::UndeclaredType(pos))
                }
            };
            match &*record_type {
                TigerType::TRecord(formals, type_id) => {
                    if typed_fields.len() > formals.len() {
                        return Err(TypeError::TooManyArguments(pos))
                    };
                    if typed_fields.len() < formals.len() {
                        return Err(TypeError::MissingRecordField(pos))
                    };
                    Ok(AST {
                        node: Exp::Record{
                            fields: formals
                                .iter()
                                .map(|(name, typ, ..)| -> Result<(Symbol, Box<AST>), TypeError> {
                                    match typed_fields.remove(name) {
                                        Some(ast) => {
                                            if *ast.typ == **typ {
                                                Ok((name.clone(), Box::new(ast)))
                                            } else {
                                                console_log!("record mismatch");
                                                Err(TypeError::TypeMismatch(pos))
                                            }
                                        }
                                        None => Err(TypeError::MissingRecordField(pos))
                                    }
                                })
                                .collect::<Result<Vec<(Symbol, Box<AST>)>, TypeError>>()?,
                            typ: record_type_symbol,
                        },
                        typ: Arc::new(TigerType::TRecord(formals.clone(), type_id.clone())),
                        pos
                    })
                },
                _ => Err(TypeError::NotRecordType(pos)),
            }
        }
        _ => panic!("delegation panic on recordexp::tipar")
    }
}