#![feature(advanced_slice_patterns, slice_patterns, exclusive_range_pattern)]
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

fn agregar_protopipo_en_env(_FunctionDec {name, params, result, ..}: &_FunctionDec, mut value_env: ValueEnviroment, type_env: &TypeEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    // Tipar el resultado
    let result_type = match result {
        None => Tipo::TUnit,
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(pos))
        }
    };
    // Ver que los parametros no se repitan
    // TODO
    let formals: Vec<Tipo> = params
        .iter()
        .map(|Field {typ, ..}: &Field| -> Result<Tipo, TypeError> {tipar_ty(typ, type_env, pos)})
        .collect::<Result<Vec<Tipo>, TypeError>>()?;
    value_env.insert(name.clone(), EnvEntry::Func {
        label: name.clone(),
        formals,
        result: result_type,
        external: false
    });
    Ok(value_env)
}

fn tipar_dec_funcion(_FunctionDec {params, result, body, ..}: &_FunctionDec,  value_env: &ValueEnviroment, type_env: &TypeEnviroment, pos: Pos) -> Result<(), TypeError> {
    // Volvemos a tipar el result porque alta paja buscarlo en el env
    let result_type = match result {
        None => Tipo::TUnit,
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(pos))
        }
    };

    // Tipar los parametros
    let params_value_env = params
        .iter()
        .try_fold(value_env.clone(), |mut prev : ValueEnviroment, Field {name, typ, ..}: &Field| -> Result<ValueEnviroment, TypeError> {
            prev.insert(name.clone(), EnvEntry::Var{
                ty: tipar_ty(typ, type_env, pos)?,
                access: Access::InReg(name.clone()),
                level: 0
            });
            Ok(prev)
        })?;

    // Tipar el body
    let body_type = tipar_exp(&*body, type_env, &params_value_env)?;
    if body_type == result_type {
        Ok(())
    }
    else {
        Err(TypeError::TypeMismatch(pos))
    }
}

fn tipar_decs_bloque_funciones(decs: &[(_FunctionDec, Pos)], type_env: &TypeEnviroment, mut value_env: ValueEnviroment) -> Result<ValueEnviroment, TypeError> {
    // Checkear que no haya funciones repetidas.

    // Armar envs con los prototipos.
    for (dec, pos) in decs {
        value_env = agregar_protopipo_en_env(dec, value_env, type_env, *pos)?;
    }

    // // Tipar los bodies.
    match decs
        .iter()
        .map(|(dec, pos): &(_FunctionDec, Pos)| -> Result<(), TypeError>{tipar_dec_funcion(dec, &value_env, type_env, *pos)})
        .collect::<Result<Vec<()>, TypeError>>() {
        Ok(_) => Ok(value_env),
        Err(type_error) => Err(type_error)
    }
    // // Si hay results, fijarse que coincidan los tipos.
    // // Devolver los envs con los prototipos.
}

fn fija_tipos(decs: &[_TypeDec], mut type_env: TypeEnviroment) -> Result<TypeEnviroment, TypeError> {
    fn gen_pairs(decs: &[_TypeDec]) -> Vec<(Symbol, Symbol)> {
        fn genp(decs: &[_TypeDec], mut res: Vec<(Symbol, Symbol)>) -> Vec<(Symbol, Symbol)> {
            match decs.split_first() {
                None => res,
                Some((_TypeDec{name, ty: Ty::Name(s_)}, rest)) => {
                    res.push((s_.clone(), name.clone()));
                    genp(rest, res)
                },
                Some((_TypeDec{name, ty: Ty::Array(s_)}, rest)) => {
                    res.push((s_.clone(), name.clone()));
                    genp(rest, res)
                },
                Some((_TypeDec{ty: Ty::Record(_), ..}, rest)) => {
                    genp(rest, res)
                },
            }
        }
        genp(decs, vec![])
    }
    fn top_sort(pairs: &[(Symbol, Symbol)]) -> Result<Vec<Symbol>, Symbol> {
        fn elements(pairs: &[(Symbol, Symbol)]) -> Vec<Symbol> {
            pairs.iter().fold(vec![], |mut l: Vec<Symbol>, (a, b): &(Symbol, Symbol)| -> Vec<Symbol> {
                let mut l1 = match l.iter().find(|&x| *x == *a) {
                    None => {l.push(a.clone()); l},
                    _ => l
                };
                match l1.iter().find(|&x| *x == *b) {
                    None => {l1.push(b.clone()); l1},
                    _ => l1
                }
            })
        }
        topological_sort::topological_sort(&elements(pairs), |n| -> Vec<Symbol> {
            pairs
                .iter()
                .filter(|(_, b)| b == n)
                .map(|(a, _)| a.clone())
                .collect::<Vec<Symbol>>()
        })
    }
    let pairs : Vec<(Symbol, Symbol)> = gen_pairs(decs);
    let orden = top_sort(&pairs);
    // Ahora hay que procesar todas las decs en este orden, agregandolas al env.
    // Esto es ignorando los Records.
    // Para agregar el tema de los records, hay que separar los que hacen ciclos y tratarlos aparte.
    Ok(type_env)
}

fn tipar_decs_bloque_tipos(decs: &[(_TypeDec, Pos)], mut  type_env: TypeEnviroment) -> Result<TypeEnviroment, TypeError> {
    for (_TypeDec {name, ty}, pos) in decs {
        type_env.insert(name.clone(), tipar_ty(&ty, &type_env, *pos)?);
    }
    Ok(type_env)
}

fn tipar_decs(decs: &[Dec], type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<(TypeEnviroment, ValueEnviroment), TypeError> {
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    for dec in decs {
        match dec {
            Dec::VarDec(vd, pos) => match tipar_dec_variable(vd, &new_type_env, new_value_env, *pos) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::FunctionDec(fd) => match tipar_decs_bloque_funciones(fd, &new_type_env, new_value_env) {
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
        Exp {node: _Exp::LetExp {decs, body}, ..} => {
            let (new_type_env, new_value_env) =  tipar_decs(decs, type_env, value_env)?;
            tipar_exp(body, &new_type_env, &new_value_env)
        },
        _ => panic!("error de delegacion en letexp::tipar"),
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}