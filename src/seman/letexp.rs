use std::convert::TryInto;

use super::super::ast::tigerabs::*;
use super::super::ast::position::Pos;
use super::tigerseman::*;

use pathfinding::directed::topological_sort;

fn type_vardec(_VarDec {name, typ, init, ..}: &_VarDec, type_env: &TypeEnviroment, mut value_env: ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    let init_type = type_exp(init, type_env, &value_env)?;
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

fn type_ty(ty: &Ty, type_env: &TypeEnviroment, pos: Pos) -> Result<Tipo, TypeError> {
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
                let field_type = type_ty(field_ty, type_env, pos)?;
                record.push((Box::new(name.clone()), Box::new(field_type), i.try_into().expect("too many fields!")));
            }
            Ok(Tipo::TRecord(record, uid::Id::new()))
        }
    }
}

fn add_prototype_to_env(_FunctionDec {name, params, result, ..}: &_FunctionDec, mut value_env: ValueEnviroment, type_env: &TypeEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    // Lookup result type in env
    let result_type = match result {
        None => Tipo::TUnit,
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(pos))
        }
    };
    // Check that argument names are not repeated
    // TODO
    let formals: Vec<Tipo> = params
        .iter()
        .map(|Field {typ, ..}: &Field| -> Result<Tipo, TypeError> {type_ty(typ, type_env, pos)})
        .collect::<Result<Vec<Tipo>, TypeError>>()?;
    value_env.insert(name.clone(), EnvEntry::Func {
        label: name.clone(),
        formals,
        result: result_type,
        external: false
    });
    Ok(value_env)
}

fn type_functiondec(_FunctionDec {params, result, body, ..}: &_FunctionDec,  value_env: &ValueEnviroment, type_env: &TypeEnviroment, pos: Pos) -> Result<(), TypeError> {
    // Lookup result type in env
    let result_type = match result {
        None => Tipo::TUnit,
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(pos))
        }
    };

    // Type the arguments
    let params_value_env = params
        .iter()
        .try_fold(value_env.clone(), |mut prev : ValueEnviroment, Field {name, typ, ..}: &Field| -> Result<ValueEnviroment, TypeError> {
            prev.insert(name.clone(), EnvEntry::Var{
                ty: type_ty(typ, type_env, pos)?,
                access: Access::InReg(name.clone()),
                level: 0
            });
            Ok(prev)
        })?;

    // Type the body
    let body_type = type_exp(&*body, type_env, &params_value_env)?;
    if body_type == result_type {
        Ok(())
    }
    else {
        Err(TypeError::TypeMismatch(pos))
    }
}

fn type_functiondec_batch(decs: &[(_FunctionDec, Pos)], type_env: &TypeEnviroment, mut value_env: ValueEnviroment) -> Result<ValueEnviroment, TypeError> {
    // Check for repeated function names

    // Add prototypes to ValueEnviroment
    for (dec, pos) in decs {
        value_env = add_prototype_to_env(dec, value_env, type_env, *pos)?;
    }

    // Type the functions with the new ValueEnviroment
    match decs
        .iter()
        .map(|(dec, pos): &(_FunctionDec, Pos)| -> Result<(), TypeError>{type_functiondec(dec, &value_env, type_env, *pos)})
        .collect::<Result<Vec<()>, TypeError>>() {
            Ok(_) => Ok(value_env),
            Err(type_error) => Err(type_error)
    }
}

fn sort_type_decs<'a>(decs: &'a [(_TypeDec, Pos)]) -> Result<Vec<&'a (_TypeDec, Pos)>, Symbol> {
    fn gen_pairs(decs: &[(_TypeDec, Pos)]) -> Vec<(Symbol, Symbol)> {
        fn genp(decs: &[(_TypeDec, Pos)], mut res: Vec<(Symbol, Symbol)>) -> Vec<(Symbol, Symbol)> {
            match decs.split_first() {
                None => res,
                Some(((_TypeDec{name, ty: Ty::Name(s_)}, _), rest)) => {
                    res.push((s_.clone(), name.clone()));
                    genp(rest, res)
                },
                Some(((_TypeDec{name, ty: Ty::Array(s_)}, _), rest)) => {
                    res.push((s_.clone(), name.clone()));
                    genp(rest, res)
                },
                Some(((_TypeDec{ty: Ty::Record(_), ..}, _), rest)) => {
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
    fn sort_decs(decs: &[(_TypeDec, Pos)], order: Vec<(Symbol)>) -> Vec<&(_TypeDec, Pos)> {
        let mut     sorted_decs = order
            .iter()
            .filter_map(|order_symbol|
                decs
                    .iter()
                    .find(|&(_TypeDec{name, ..}, _)| *name == *order_symbol)
            )
            .collect::<Vec<&(_TypeDec, Pos)>>();
        sorted_decs.reverse();
        sorted_decs
    }
    let pairs : Vec<(Symbol, Symbol)> = gen_pairs(decs);
    println!("Pairs {:?}", pairs);
    let order = top_sort(&pairs)?;
    println!("Order {:?}", order);
    Ok(sort_decs(decs, order))
    // Esto es ignorando los Records.
    // Para agregar el tema de los records, hay que separar los que hacen ciclos y tratarlos aparte.
}

fn type_typedec_block(decs: &[(_TypeDec, Pos)], mut  type_env: TypeEnviroment) -> Result<TypeEnviroment, TypeError> {
    let sorted_decs = match sort_type_decs(decs) {
        Ok(sd) => sd,
        Err(s) => return Err(TypeError::TypeCycle(
            // This is the Pos of the dec corresponding to the cycle error
            decs.iter().find(|&(_TypeDec{name, ..}, _)| *name == s).expect("sorted symbol must be on decs").1)
        )
    };
    println!("Sorted decs {:?}", sorted_decs);
    for (_TypeDec {name, ty}, pos) in sorted_decs {
        type_env.insert(name.clone(), type_ty(&ty, &type_env, *pos)?);
    }
    Ok(type_env)
}

fn type_decs(decs: &[Dec], type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<(TypeEnviroment, ValueEnviroment), TypeError> {
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    for dec in decs {
        match dec {
            Dec::VarDec(vd, pos) => match type_vardec(vd, &new_type_env, new_value_env, *pos) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::FunctionDec(fd) => match type_functiondec_batch(fd, &new_type_env, new_value_env) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::TypeDec(td) => match type_typedec_block(td, new_type_env) {
                Ok(tenv) => new_type_env = tenv,
                Err(type_error) => return Err(type_error),
            },
        };

    }
    Ok((new_type_env, new_value_env))
}

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp {
        Exp {node: _Exp::LetExp {decs, body}, ..} => {
            let (new_type_env, new_value_env) =  type_decs(decs, type_env, value_env)?;
            type_exp(body, &new_type_env, &new_value_env)
        },
        _ => panic!("error de delegacion en letexp::tipar"),
    }
}

pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}