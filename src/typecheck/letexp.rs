use crate::ast::*;
use crate::typecheck::*;
use pathfinding::directed::topological_sort;
use std::convert::TryInto;

fn typecheck_vardec(
    _VarDec {
        name, typ, init, ..
    }: &_VarDec,
    type_env: &TypeEnviroment,
    mut value_env: ValueEnviroment,
    pos: Pos,
) -> Result<ValueEnviroment, TypeError> {
    let init_type = type_exp(init, type_env, &value_env)?;
    let dec_type = match typ {
        None => init_type,
        Some(typ_string) => match type_env.get(typ_string) {
            Some(table_type) => {
                if **table_type == *init_type {
                    (*table_type).clone()
                } else {
                    return Err(TypeError::TypeMismatch(pos));
                }
            }
            None => return Err(TypeError::UndeclaredType(pos)),
        },
    };
    value_env.insert(name.clone(), EnvEntry::Var { ty: dec_type });
    Ok(value_env)
}

fn ty_to_tigertype(ty: &Ty, type_env: &TypeEnviroment, pos: Pos) -> Result<Arc<TigerType>, TypeError> {
    match ty {
        Ty::Name(symbol) => match type_env.get(symbol) {
            Some(tipo) => Ok((*tipo).clone()),
            None => Err(TypeError::UndeclaredType(pos)),
        },
        Ty::Array(symbol) => match type_env.get(symbol) {
            Some(tipo) => Ok(Arc::new(TigerType::TArray(
                (*tipo).clone(),
                uid::Id::new(),
            ))),
            None => Err(TypeError::UndeclaredType(pos)),
        },
        Ty::Record(fields_vector) => {
            let mut record: Vec<(String, Arc<TigerType>, u8)> = vec![];
            for (i, Field {name,typ: field_ty, ..}) in fields_vector.iter().enumerate() {
                record.push((
                    name.clone(),
                    ty_to_tigertype(field_ty, type_env, pos)?,
                    i.try_into().expect("too many fields!"),
                ));
            }
            // let type_id = match type_env.remove(k: &Q)
            Ok(Arc::new(TigerType::TRecord(record, uid::Id::new())))
        }
    }
}

fn add_prototype_to_env(
    _FunctionDec {
        name,
        params,
        result,
        ..
    }: &_FunctionDec,
    mut value_env: ValueEnviroment,
    type_env: &TypeEnviroment,
    pos: Pos,
) -> Result<ValueEnviroment, TypeError> {
    // Lookup result type in env
    let result_type : Arc<TigerType> = match result {
        None => Arc::new(TigerType::TUnit),
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(pos)),
        },
    };
    // Check that argument names are not repeated
    // TODO
    let formals: Vec<Arc<TigerType>> = params
        .iter()
        .map(
            |Field { typ, .. }: &Field| -> Result<Arc<TigerType>, TypeError> {
                ty_to_tigertype(typ, type_env, pos)
            },
        )
        .collect::<Result<Vec<Arc<TigerType>>, TypeError>>()?;
    value_env.insert(
        name.clone(),
        EnvEntry::Func {
            formals,
            result: result_type,
        },
    );
    Ok(value_env)
}

fn typecheck_functiondec(
    _FunctionDec {
        params,
        result,
        body,
        ..
    }: &_FunctionDec,
    value_env: &ValueEnviroment,
    type_env: &TypeEnviroment,
    pos: Pos,
) -> Result<(), TypeError> {
    // Lookup result type in env
    let result_type : Arc<TigerType> = match result {
        None => Arc::new(TigerType::TUnit),
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => return Err(TypeError::UndeclaredType(pos)),
        },
    };

    // Type the arguments
    let params_value_env = params.iter().try_fold(
        value_env.clone(),
        |mut prev: ValueEnviroment,
         Field { name, typ, .. }: &Field|
         -> Result<ValueEnviroment, TypeError> {
            prev.insert(
                name.clone(),
                EnvEntry::Var {
                    ty: ty_to_tigertype(typ, type_env, pos)?,
                },
            );
            Ok(prev)
        },
    )?;

    // Type the body
    let body_type = type_exp(&*body, type_env, &params_value_env)?;
    if body_type == result_type {
        Ok(())
    } else {
        Err(TypeError::TypeMismatch(pos))
    }
}

fn typecheck_functiondec_batch(
    decs: &[(_FunctionDec, Pos)],
    type_env: &TypeEnviroment,
    mut value_env: ValueEnviroment,
) -> Result<ValueEnviroment, TypeError> {
    // Check for repeated function names
    // TODO

    // Add prototypes to ValueEnviroment
    for (dec, pos) in decs {
        value_env = add_prototype_to_env(dec, value_env, type_env, *pos)?;
    }

    // Type the functions with the new ValueEnviroment
    match decs
        .iter()
        .map(
            |(dec, pos): &(_FunctionDec, Pos)| -> Result<(), TypeError> {
                typecheck_functiondec(dec, &value_env, type_env, *pos)
            },
        )
        .collect::<Result<Vec<()>, TypeError>>()
    {
        Ok(_) => Ok(value_env),
        Err(type_error) => Err(type_error),
    }
}

fn sort_type_decs(decs: &[(_TypeDec, Pos)]) -> Result<(Vec<&(_TypeDec, Pos)>, Vec<&(_TypeDec, Pos)>), Symbol> {
    // This function takes a batch of type declarations and returns (if posible):
    // - A vector of the type declarations that are not records, topologically sorted.
    // - A vector of record type declarations.
    // If it fails, it return the symbol of the not record type with cyclic declaration.
    fn gen_pairs(decs: &[(_TypeDec, Pos)]) -> (Vec<(Symbol, Symbol)>, Vec<Symbol>) {
        // gen_pairs is a wrapper for genp.
        // gen_pairs only calls genp with an initial state.
        fn genp(decs: &[(_TypeDec, Pos)], mut res: Vec<(Symbol, Symbol)>, mut records: Vec<Symbol>, ) -> (Vec<(Symbol, Symbol)>, Vec<Symbol>) {
            // This recursive function takes a batch of function declarations and returns
            // - A vector of pairs of symbols where left declaration depends on right declaration (or oposite?)
            // - A vector of symbols of record declarations.
            //
            // This could have been a map, I think.
            match decs.split_first() {
                None => (res, records),
                Some(((_TypeDec {name, ty: Ty::Name(s_), .. }, _, ), rest)) => {
                    res.push((s_.clone(), name.clone()));
                    genp(rest, res, records)
                }
                Some(((_TypeDec {name, ty: Ty::Array(s_), ..}, _), rest)) => {
                    res.push((s_.clone(), name.clone()));
                    genp(rest, res, records)
                }
                Some(((_TypeDec {name: record_name, ty: Ty::Record(..), ..}, _,), rest,)) => {
                    records.push(record_name.clone());
                    genp(rest, res, records)
                }
            }
        }
        genp(decs, vec![], vec![])
    }
    fn top_sort(pairs: &[(Symbol, Symbol)]) -> Result<Vec<Symbol>, Symbol> {
        fn elements(pairs: &[(Symbol, Symbol)]) -> Vec<Symbol> {
            pairs.iter().fold(
                vec![],
                |mut l: Vec<Symbol>, (a, b): &(Symbol, Symbol)| -> Vec<Symbol> {
                    let mut l1 = match l.iter().find(|&x| *x == *a) {
                        None => {
                            l.push(a.clone());
                            l
                        }
                        _ => l,
                    };
                    match l1.iter().find(|&x| *x == *b) {
                        None => {
                            l1.push(b.clone());
                            l1
                        }
                        _ => l1,
                    }
                },
            )
        }
        topological_sort::topological_sort(&elements(pairs), |n| -> Vec<Symbol> {
            pairs
                .iter()
                .filter(|(_, b)| b == n)
                .map(|(a, _)| a.clone())
                .collect::<Vec<Symbol>>()
        })
    }
    fn sort_decs(decs: &[(_TypeDec, Pos)], order: Vec<(Symbol)>, record_names: Vec<(Symbol)>) -> (Vec<&(_TypeDec, Pos)>, Vec<&(_TypeDec, Pos)>) {
        let mut sorted_decs = order
            .iter()
            .filter_map(|order_symbol| {
                decs.iter()
                    .find(|&(_TypeDec { name, .. }, _)| *name == *order_symbol)
            })
            .collect::<Vec<&(_TypeDec, Pos)>>();
        sorted_decs.reverse();
        let records = record_names
            .iter()
            .filter_map(|record_symbol| {
                decs.iter()
                    .find(|&(_TypeDec { name, .. }, _)| *name == *record_symbol)
            })
            .collect::<Vec<&(_TypeDec, Pos)>>();
        (sorted_decs, records)
    }
    let (pairs, records) = gen_pairs(decs);
    let order = top_sort(&pairs)?;
    Ok(sort_decs(decs, order, records))
}

fn typecheck_typedec_block(
    decs: &[(_TypeDec, Pos)],
    mut type_env: TypeEnviroment,
) -> Result<TypeEnviroment, TypeError> {
    // Sort by type dependency
    let (sorted_decs, records) = match sort_type_decs(decs) {
        Ok((sd, rr)) => (sd, rr),
        Err(s) => {
            return Err(TypeError::TypeCycle(
                // This is the Pos of the dec corresponding to the cycle error
                decs.iter()
                    .find(|&(_TypeDec { name, .. }, _)| *name == s)
                    .expect("sorted symbol must be on decs")
                    .1,
            ))
        }
    };
    // Insert placeholders for recursive records in TypeEnviroment
    records
        .iter()
        .for_each(|(_TypeDec { name, .. }, _)| {
            type_env.insert(name.clone(), Arc::new(TigerType::TRecord(vec![], TypeId::new())));
        });
    // Insert declarations, except recursive records.
    // Problem: Record types have boxes on fields.
    // There is no posible representation of recursive records with this AST
    for (_TypeDec { name, ty, .. }, pos) in sorted_decs {
        type_env.insert(name.clone(), ty_to_tigertype(&ty, &type_env, *pos)?);
    }

    for (_TypeDec { name, ty }, pos) in records {
            match ty {
                Ty::Record(fields_vector) => {
                    let mut record: Vec<(String, Arc<TigerType>, u8)> = vec![];
                    for (i, Field {name,typ: field_ty, ..}) in fields_vector.iter().enumerate() {
                        record.push((
                            name.clone(),
                            ty_to_tigertype(field_ty, &type_env, *pos)?,
                            i.try_into().expect("too many fields!"),
                        ));
                    }
                    let type_id = match type_env.remove(name).as_deref() {
                        Some(TigerType::TRecord(_, id)) => *id,
                        _ => panic!("There should be a record header in the env")
                    };
                    type_env.insert(name.clone(), Arc::new(TigerType::TRecord(record, type_id)));
                }
                _ => panic!("There should only be records in this vector")
            }
    }
    Ok(type_env)
}

fn typecheck_decs(
    decs: &[Dec],
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<(TypeEnviroment, ValueEnviroment), TypeError> {
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    for dec in decs {
        match dec {
            Dec::VarDec(vd, pos) => {
                match typecheck_vardec(vd, &new_type_env, new_value_env, *pos) {
                    Ok(venv) => new_value_env = venv,
                    Err(type_error) => return Err(type_error),
                }
            }
            Dec::FunctionDec(fd) => {
                match typecheck_functiondec_batch(fd, &new_type_env, new_value_env) {
                    Ok(venv) => new_value_env = venv,
                    Err(type_error) => return Err(type_error),
                }
            }
            Dec::TypeDec(td) => match typecheck_typedec_block(td, new_type_env) {
                Ok(tenv) => new_type_env = tenv,
                Err(type_error) => return Err(type_error),
            },
        };
    }
    Ok((new_type_env, new_value_env))
}

pub fn typecheck(
    exp: &Exp,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<Arc<TigerType>, TypeError> {
    match exp {
        Exp {
            node: _Exp::Let { decs, body },
            ..
        } => {
            let (new_type_env, new_value_env) = typecheck_decs(decs, type_env, value_env)?;
            type_exp(body, &new_type_env, &new_value_env)
        }
        _ => panic!("error de delegacion en letexp::tipar"),
    }
}
