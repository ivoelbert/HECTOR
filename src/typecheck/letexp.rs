use crate::ast::*;
use crate::typecheck::*;
use pathfinding::directed::topological_sort;
use std::convert::TryInto;
use crate::utils::log;

fn typecheck_vardec(
    _VarDec {
        name, typ, init, escape
    }: _VarDec,
    type_env: &TypeEnviroment,
    mut value_env: ValueEnviroment,
    pos: Pos,
) -> Result<(_VarDec, ValueEnviroment), TypeError> {
    let init_ast = type_exp(*init, type_env, &value_env)?;
    let dec_type = if let Some(typ_symbol) = &typ {
        match type_env.get(typ_symbol) {
            Some(table_type) => {
                if **table_type == *init_ast.typ {
                    (*table_type).clone()
                } else {
                    console_log!("let vardec mismatch");
                    return Err(TypeError::TypeMismatch(pos));
                }
            }
            None => {
                console_log!("let vardec undeclared");
                return Err(TypeError::UndeclaredType(pos))
            }
        }
    } else {
        init_ast.typ.clone()
    };
    value_env.insert(name.clone(), EnvEntry::Var { ty: dec_type });
    Ok((_VarDec {
        name,
        typ,
        init: Box::new(init_ast),
        escape
    }, value_env))
}

fn ty_to_tigertype(ty: &Ty, type_env: &TypeEnviroment, pos: Pos) -> Result<Arc<TigerType>, TypeError> {
    match ty {
        Ty::Name(symbol) => match type_env.get(symbol) {
            Some(tipo) => Ok((*tipo).clone()),
            None => {
                console_log!("let ty name undeclared - ty: {:?}, type_env: {:?}", ty, type_env);
                Err(TypeError::UndeclaredType(pos))
            },
        },
        Ty::Array(symbol) => match type_env.get(symbol) {
            Some(tipo) => Ok(Arc::new(TigerType::TArray(
                (*tipo).clone(),
                newtypeid(),
            ))),
            None => {
                console_log!("let ty array undeclared");
                Err(TypeError::UndeclaredType(pos))
            },
        },
        Ty::Record(fields_vector) => {
            let mut record: Vec<(String, Arc<TigerType>, i64)> = vec![];
            for (i, Field {name,typ: field_ty, ..}) in fields_vector.iter().enumerate() {
                record.push((
                    name.clone(),
                    ty_to_tigertype(field_ty, type_env, pos)?,
                    i.try_into().expect("too many fields!"),
                ));
            }
            // let type_id = match type_env.remove(k: &Q)
            Ok(Arc::new(TigerType::TRecord(record, newtypeid())))
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
            None => {
                console_log!("let prototype undeclared");
                return Err(TypeError::UndeclaredType(pos))
            },
        },
    };
    // Check that argument names are not repeated
    let names : Vec<&String> = params
        .iter()
        .map(|Field { name, .. }: &Field| name)
        .collect();

    if (1..names.len()).any(|i| names[i..].contains(&names[i - 1])) {
        return Err(TypeError::DuplicatedDefinitions(pos))
    }

    // get parameter types
    let formals: Vec<Arc<TigerType>> = params
        .iter()
        .map(
            |Field { typ, .. }: &Field| -> Result<Arc<TigerType>, TypeError> {
                ty_to_tigertype(typ, type_env, pos)
            },
        )
        .collect::<Result<Vec<Arc<TigerType>>, TypeError>>()?;
    // Insert in env
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
        name,
    }: _FunctionDec,
    value_env: &ValueEnviroment,
    type_env: &TypeEnviroment,
    pos: Pos,
) -> Result<_FunctionDec, TypeError> {
    // Lookup result type in env
    let result_type : Arc<TigerType> = match &result {
        None => Arc::new(TigerType::TUnit),
        Some(result_name) => match type_env.get(result_name) {
            Some(result_table_type) => result_table_type.clone(),
            None => {
                console_log!("let typecheck functiondec undeclared");
                return Err(TypeError::UndeclaredType(pos))
            },
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
    let body_ast = type_exp(*body, type_env, &params_value_env)?;
    if body_ast.typ == result_type {
        Ok(_FunctionDec {
            name,
            params,
            result,
            body: Box::new(body_ast)
        })
    } else {
        console_log!("let functiondec mismatch");
        Err(TypeError::TypeMismatch(pos))
    }
}

fn typecheck_functiondec_batch(
    decs: Vec<(_FunctionDec, Pos)>,
    type_env: &TypeEnviroment,
    mut value_env: ValueEnviroment,
) -> Result<(Vec<(_FunctionDec, Pos)>, ValueEnviroment), TypeError> {

    // Add prototypes to ValueEnviroment
    let mut added_names = vec![];
    for (dec, pos) in &decs {
        // Check for repeated names, in a very inneficient way.
        if added_names.contains(&dec.name) {
            return Err(TypeError::DuplicatedDefinitions(*pos))
        }
        value_env = add_prototype_to_env(dec, value_env, type_env, *pos)?;
        added_names.push(dec.name.clone())
    }

    // Type the functions with the new ValueEnviroment
    Ok((decs
        .into_iter()
        .map(
            |(dec, pos): (_FunctionDec, Pos)| -> Result<(_FunctionDec, Pos), TypeError> {
                Ok((typecheck_functiondec(dec, &value_env, type_env, pos)?, pos))
            },
        )
        .collect::<Result<Vec<(_FunctionDec, Pos)>, TypeError>>()?, value_env))
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

fn typecheck_typedec_batch(
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
    let mut added_names = vec![];
    for (_TypeDec { name, .. }, pos) in &records {
        type_env.insert(name.clone(), Arc::new(TigerType::TRecord(vec![], TypeId::new())));
    }
    // Insert declarations, except recursive records.
    for (_TypeDec { name, ty, .. }, pos) in sorted_decs {
        // Check for repeated names, ineficiently.
        if added_names.contains(name) {
            return Err(TypeError::DuplicatedDefinitions(*pos))
        }
        type_env.insert(name.clone(), ty_to_tigertype(&ty, &type_env, *pos)?);
        added_names.push(name.clone())
    }

    for (_TypeDec { name, ty }, pos) in records {
            match ty {
                Ty::Record(fields_vector) => {
                    let mut record: Vec<(String, Arc<TigerType>, i64)> = vec![];
                    for (i, Field {name,typ: field_ty, ..}) in fields_vector.iter().enumerate() {
                        record.push((
                            name.clone(),
                            ty_to_tigertype(field_ty, &type_env, *pos)?,
                            i.try_into().expect("too many fields!"),
                        ));
                    }
                    let type_id = match type_env.remove(name).as_deref() {
                        Some(TigerType::TRecord(_, id)) => id.clone(),
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
    decs: Vec<Dec>,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<(Vec<Dec>, TypeEnviroment, ValueEnviroment), TypeError> {
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    let mut typed_decs : Vec<Dec> = vec![];
    for dec in decs {
        match dec {
            Dec::VarDec(vd, pos) => {
                let (typed_vd, venv) = typecheck_vardec(vd, &new_type_env, new_value_env, pos)?;
                new_value_env = venv;
                typed_decs.push(Dec::VarDec(typed_vd, pos));
            }
            Dec::FunctionDec(fd) => {
                let (typed_fd, venv) = typecheck_functiondec_batch(fd, &new_type_env, new_value_env)?;
                new_value_env = venv;
                typed_decs.push(Dec::FunctionDec(typed_fd));
            }
            Dec::TypeDec(td) => new_type_env = typecheck_typedec_batch(&td, new_type_env)?
        };
    }
    Ok((typed_decs, new_type_env, new_value_env))
}

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    match node {
        Exp::Let { decs, body } => {
            let (typed_decs, new_type_env, new_value_env) = typecheck_decs(decs, type_env, value_env)?;
            let body_ast = type_exp(*body, &new_type_env, &new_value_env)?;
            let typ = body_ast.typ.clone();
            Ok(AST {
                node: Exp::Let {
                    decs: typed_decs,
                    body: Box::new(body_ast)
                },
                typ,
                pos,
            })
        }
        _ => panic!("error de delegacion en letexp::tipar"),
    }
}
