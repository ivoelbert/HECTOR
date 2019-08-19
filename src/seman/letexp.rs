use std::convert::TryInto;

use super::super::ast::tigerabs::*;
use super::super::ast::position::Pos;
use super::tigerseman::*;

use pathfinding::directed::topological_sort;

fn tipar_dec_variable(_VarDec {name, typ, init, ..}: &_VarDec, type_env: &TypeEnviroment, mut value_env: ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    let init_type = tipar_exp(init, type_env, &value_env)?;
    let dec_type = match typ {
        None => init_type,
        Some(typ_string) => match type_env.get(typ_string) {
            Some(table_type) =>
                if *table_type == init_type {
                    table_type.clone()
                } else {
                    return Err(TypeError::TypeMismatch(pos))
                },
            None => return Err(TypeError::UndeclaredType(pos))
        }
    };
    value_env.insert(name.clone(), EnvEntry::Var {
        ty: dec_type,
        access: Access::InFrame(0),
        level: 0
    });
    Ok(value_env)
}

fn tipar_ty(ty: &Ty, type_env: &TypeEnviroment, pos: Pos) -> Result<Tipo, TypeError> {
    match ty {
        Ty::Name(symbol) => match type_env.get(symbol) {
            Some(tipo) => Ok(tipo.clone()),
            None => Err(TypeError::UndeclaredType(pos))
        },
        Ty::Array(symbol) => match type_env.get(symbol) {
            Some(tipo) => Ok(Tipo::TArray(Box::new(tipo.clone()), uid::Id::new())),
            None => Err(TypeError::UndeclaredType(pos))
        },
        Ty::Record(fields_vector) => {
            let mut record : Vec<(Box<String>, Box<Tipo>, u8)> = vec![];
            for (i, Field {name, typ : field_ty, ..}) in fields_vector.iter().enumerate()  {
                let field_type = tipar_ty(field_ty, type_env, pos)?;
                record.push((Box::new(name.clone()), Box::new(field_type), i.try_into().expect("too many fields!")));
            }
            Ok(Tipo::TRecord(record, uid::Id::new()))
        }
    }
}

fn tipar_dec_funcion(_FunctionDec {name, params, result, body, pos}: &_FunctionDec, mut value_env: ValueEnviroment, type_env: &TypeEnviroment) -> Result<ValueEnviroment, TypeError> {

    // Tipar el resultado
    let result_type = match result {
        None => Tipo::TUnit,
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(*pos))
        }
    };
    // Ver que los parametros no se repitan
    // TODO

    // Tipar los parametros
    let params_value_env = params
        .iter()
        .try_fold(value_env.clone(), |mut prev : ValueEnviroment, Field {name, typ, ..}: &Field| -> Result<ValueEnviroment, TypeError> {
            prev.insert(name.clone(), EnvEntry::Var{
                ty: tipar_ty(typ, type_env, *pos)?,
                access: Access::InReg(name.clone()),
                level: 0
            });
            Ok(prev)
        })?;
    let formals: Vec<Tipo> = params
        .iter()
        .map(|Field {typ, ..}: &Field| -> Result<Tipo, TypeError> {tipar_ty(typ, type_env, *pos)})
        .collect::<Result<Vec<Tipo>, TypeError>>()?;

    // Tipar el body
    let body_type = tipar_exp(&*body, type_env, &params_value_env)?;
    if body_type == result_type {
        // Insertar en el env
        value_env.insert(name.clone(), EnvEntry::Func {
            label: name.clone(),
            formals,
            result: result_type,
            external: false
        });
        Ok(value_env)
    }
    else {
        Err(TypeError::TypeMismatch(*pos))
    }
}

fn sort_types(decs: Vec<_TypeDec>, type_env: &TypeEnviroment) -> Result<Vec<_TypeDec>, _TypeDec> {
    // Esto eventualmente debería hacer un sort topologico.
    // Encuentra ciclos.
    // topological_sort::topological_sort(&decs, |_TypeDec {ty, ..}: _TypeDec| {
    //     match ty {
    //         Ty::Name(s) => vec![],
    //         Ty::Record(fields) => vec![],
    //         Ty::Array(s) => vec![],
    //     }
    // })
    Ok(decs)
}

fn tipar_decs_bloque_funciones(decs: &[_FunctionDec], type_env: &TypeEnviroment, mut value_env: ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    // Checkear que no haya funciones repetidas.

    // Armar envs con los prototipos.
    // for _FunctionDec { name, params, result, body} in &decs {
    //     new_value_env.insert(name.clone(), EnvEntry::Func {
    //         label: name.clone(),
    //         formals: params.iter().map(|field: &Field| -> Result<Tipo, TypeError> {
    //             match field.typ {
    //                 Ty::NameTy(type_symbol) =>  match type_env.get(&type_symbol) {
    //                         Some(arg_type) => Ok(*arg_type),
    //                         None => Err(TypeError::UndeclaredType(pos)),
    //                     },
    //                 Ty::RecordTy(fields) => Ok(Tipo::TUnit),
    //                 Ty::ArrayTy(symbol) => Ok(Tipo::TUnit),
    //             }
    //         }).collect()? ,
    //         result: match result {
    //             Some(result_type_symbol) => match type_env.get(result_type_symbol) {
    //                 Some(result_type) => *result_type,
    //                 None => return Err(TypeError::UndeclaredType(pos)),
    //             },
    //             None => Tipo::TUnit
    //         },
    //         external: false
    //     });
    // }

    // // Tipar los bodies.
    for dec in decs {
        value_env = tipar_dec_funcion(dec, value_env, type_env)?;
    }
    Ok(value_env)
    // // Si hay results, fijarse que coincidan los tipos.
    // // Devolver los envs con los prototipos.
}

fn tipar_decs_bloque_tipos(decs: &[_TypeDec], mut  type_env: TypeEnviroment) -> Result<TypeEnviroment, TypeError> {
    //let sorted_decs = sort_types(decs, &type_env);
    for _TypeDec {name, ty, pos} in decs {
        type_env.insert(name.clone(), tipar_ty(&ty, &type_env, *pos)?);
    }
    Ok(type_env)
}

fn tipar_decs(decs: &[Dec], type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<(TypeEnviroment, ValueEnviroment), TypeError> {
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    for dec in decs {
        match dec {
            Dec::VarDec(vd) => match tipar_dec_variable(vd, &new_type_env, new_value_env, pos) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::FunctionDec(fd) => match tipar_decs_bloque_funciones(fd, &new_type_env, new_value_env, pos) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::TypeDec(td) => match tipar_decs_bloque_tipos(td, new_type_env) {
                Ok(tenv) => new_type_env = tenv,
                Err(type_error) => return Err(type_error),
            },
        };

    }
    Ok((new_type_env, new_value_env))
}

pub fn tipar(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp {
        Exp {node: _Exp::LetExp {decs, body}, pos} => {
            let (new_type_env, new_value_env) =  tipar_decs(decs, type_env, value_env, *pos)?;
            tipar_exp(body, &new_type_env, &new_value_env)
        },
        _ => panic!("error de delegacion en letexp::tipar"),
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}