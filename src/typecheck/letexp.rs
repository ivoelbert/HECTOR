use crate::ast::*;
use crate::typecheck::*;
use pathfinding::directed::topological_sort;
use std::convert::TryInto;

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
        if let Some(table_type) = type_env.get(typ_symbol) {
            if **table_type == *init_ast.typ {
                (*table_type).clone()
            } else {
                console_log!("let vardec mismatch");
                return Err(TypeError::TypeMismatch(pos));
            }
        } else {
            console_log!("let vardec undeclared");
            return Err(TypeError::UndeclaredType(pos))
        }
    } else {
        if let TigerType::TNil = *init_ast.typ {
            return Err(TypeError::UnconstrainedNilInitialization(pos))
        };
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
            let mut record: Vec<(String, Arc<TigerType>, i32)> = vec![];
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
    let names = decs.iter().map(|(_FunctionDec{name, ..}, ..)| -> String {name.clone()}).collect::<Vec<String>>();
    if (1..decs.len()).any(|i| names[i..].contains(&names[i - 1])) {
        return Err(TypeError::DuplicatedDefinitions(decs[0].1))
    }

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

type DecList<'a> = Vec<&'a (_TypeDec, Pos)>;

fn sort_type_decs(decs: &[(_TypeDec, Pos)]) -> Result<(DecList, DecList), Symbol> {
    /// This function takes a batch of type declarations and returns (if posible):
    /// - A vector of the type declarations that are not records, topologically sorted.
    /// - A vector of record type declarations.
    /// If it fails, it return the symbol of the not record type with cyclic declaration.
    fn gen_pairs(decs: &[(_TypeDec, Pos)]) -> (Vec<(Symbol, Symbol)>, Vec<Symbol>) {
        /// gen_pairs is a wrapper for genp.
        /// gen_pairs only calls genp with an initial state.
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
    fn sort_decs(decs: &[(_TypeDec, Pos)], order: Vec<Symbol>, record_names: Vec<Symbol>) -> (DecList, DecList) {
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
    let names = decs.iter().map(|(_TypeDec{name, ..}, ..)| -> String {name.clone()}).collect::<Vec<String>>();
    if (1..decs.len()).any(|i| names[i..].contains(&names[i - 1])) {
        return Err(TypeError::DuplicatedDefinitions(decs[0].1))
    }
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
    for (_TypeDec { name, .. }, ..) in &records {
        type_env.insert(name.clone(), Arc::new(TigerType::TRecord(vec![], TypeId::new())));
    }
    // Insert declarations, except recursive records.
    for (_TypeDec { name, ty, .. }, pos) in sorted_decs {
        type_env.insert(name.clone(), ty_to_tigertype(&ty, &type_env, *pos)?);
    }

    for (_TypeDec { name, ty }, pos) in records {
            match ty {
                Ty::Record(fields_vector) => {
                    let mut record: Vec<(String, Arc<TigerType>, i32)> = vec![];
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
    decs: Vec<Dec>,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<(Vec<Dec>, TypeEnviroment, ValueEnviroment), TypeError> {
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    let mut typed_decs : Vec<Dec> = vec![];
    for dec in decs {
        match dec {
            Dec::Var(vd, pos) => {
                let (typed_vd, venv) = typecheck_vardec(vd, &new_type_env, new_value_env, pos)?;
                new_value_env = venv;
                typed_decs.push(Dec::Var(typed_vd, pos));
            }
            Dec::Function(fd) => {
                let (typed_fd, venv) = typecheck_functiondec_batch(fd, &new_type_env, new_value_env)?;
                new_value_env = venv;
                typed_decs.push(Dec::Function(typed_fd));
            }
            Dec::Type(td) => new_type_env = typecheck_typedec_batch(&td, new_type_env)?
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

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    fn boxed_var(kind: VarKind) -> Box<Var> {
        Box::new(Var {kind, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)})
    }
    #[test]
    #[wasm_bindgen_test]
    fn letexp_vardec_no_type_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Var(
                _VarDec::new(
                    Symbol::from("foo"),
                    None,
                    boxed_ast(Exp::Int(4))
                ),
                Pos{line: 0, column: 0}
            )],
            body: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_vardec_type_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Var(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Int(4)),
                ),
                Pos{line: 0, column: 0}
            )],
            body: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();

        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_vardec_undeclared_type() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Var(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("un_tipo_no_declarado")),
                    boxed_ast(Exp::Int(4)),
                ),
                Pos{line: 0, column: 0}
            )],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_vardec_type_mismatch() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Var(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("string")),
                    boxed_ast(Exp::Int(4))
                ),
                Pos{line: 0, column: 0}
            )],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TypeMismatch(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_name_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![
                Dec::Type(vec![(
                    _TypeDec::new(
                        Symbol::from("FooType"),
                        Ty::Name(Symbol::from("int"))
                    ),
                    Pos{line: 0, column: 0}
                )]),
                Dec::Var(
                    _VarDec::new(
                        Symbol::from("foo"),
                        Some(Symbol::from("FooType")),
                        boxed_ast(Exp::Int(4))
                    ),
                    Pos{line: 0, column: 0}
                ),
            ],
            body: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_array_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![
                Dec::Type(vec![(
                    _TypeDec::new(
                        Symbol::from("FooType"),
                        Ty::Array(Symbol::from("int"))
                    ),
                    Pos{line: 0, column: 0}
                )]),
                Dec::Var(
                    _VarDec::new(
                        Symbol::from("foo"),
                        Some(Symbol::from("FooType")),
                        boxed_ast(Exp::Array {
                            typ: Symbol::from("FooType"),
                            size:boxed_ast(Exp::Int(1)),
                            init: boxed_ast(Exp::Int(2)),
                        })
                    ),
                    Pos{line: 0, column: 0}
                ),
            ],
            body: boxed_ast(Exp::Var(
                make_var(VarKind::Subscript(
                    boxed_var(VarKind::Simple(Symbol::from("foo"))),
                    boxed_ast(Exp::Int(0))
                ))
            ))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_record_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![
                Dec::Type(vec![(
                    _TypeDec::new(
                        Symbol::from("FooType"),
                        Ty::Record(vec![
                            Field {
                                name: Symbol::from("bar"),
                                typ: Ty::Name(Symbol::from("int")),
                                escape: false,
                            }
                        ])
                    ),
                    Pos{line: 0, column: 1}
                )]),
                Dec::Var(
                    _VarDec::new(
                        Symbol::from("foo"),
                        Some(Symbol::from("FooType")),
                        boxed_ast(Exp::Record {
                            fields: vec![(Symbol::from("bar"), boxed_ast(Exp::Int(1)))],
                            typ: Symbol::from("FooType"),
                        })
                    ),
                    Pos{line: 0, column: 2}
                )],
            body: boxed_ast(Exp::Var(
                make_var(VarKind::Field(
                    boxed_var(VarKind::Simple(Symbol::from("foo"))),
                    Symbol::from("bar")
                ))
            ))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_infinite_recursion() {
       let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Type(vec![
                (_TypeDec::new(Symbol::from("FooType"), Ty::Name(Symbol::from("BaazType"))), Pos{line: 0, column: 0}),
                (_TypeDec::new(Symbol::from("BaazType"), Ty::Name(Symbol::from("FooType"))), Pos{line: 0, column: 0}),
            ])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TypeCycle(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
    #[test]
    #[wasm_bindgen_test]
    fn test_typedec_recursive_ok() {
       let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Type(vec![
                (_TypeDec::new(Symbol::from("C"), Ty::Name(Symbol::from("B"))), Pos{line: 0, column: 0}),
                (_TypeDec::new(Symbol::from("B"), Ty::Name(Symbol::from("A"))), Pos{line: 0, column: 0}),
                (_TypeDec::new(Symbol::from("A"), Ty::Name(Symbol::from("int"))), Pos{line: 0, column: 0}),
            ])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(..) => panic!("type error"),
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_typedec_undeclared_type() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Type(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Name(Symbol::from("BaazType"))
                ),
                Pos{line: 0, column: 0}
            )])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn record_type_cycle_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![
                Dec::Type(vec![(
                    _TypeDec::new(
                        Symbol::from("List"),
                        Ty::Record(vec![
                            Field {
                                name: Symbol::from("head"),
                                typ: Ty::Name(Symbol::from("int")),
                                escape: false,
                            },
                            Field {
                                name: Symbol::from("tail"),
                                typ: Ty::Name(Symbol::from("List")),
                                escape: false,
                            }
                        ])
                    ),
                    Pos{line: 0, column: 1}
                )]),
                Dec::Var(
                    _VarDec::new(
                        Symbol::from("foo"),
                        Some(Symbol::from("List")),
                        boxed_ast(Exp::Record {
                            fields: vec![
                                (Symbol::from("head"), boxed_ast(Exp::Int(1))),
                                (Symbol::from("tail"), boxed_ast(Exp::Record {
                                    fields: vec![
                                        (Symbol::from("head"), boxed_ast(Exp::Int(2))),
                                        (Symbol::from("tail"), boxed_ast(Exp::Record {
                                            fields: vec![
                                                (Symbol::from("head"), boxed_ast(Exp::Int(3))),
                                                (Symbol::from("tail"), boxed_ast(Exp::Record {
                                                    fields: vec![
                                                        (Symbol::from("head"), boxed_ast(Exp::Int(4))),
                                                        (Symbol::from("tail"), boxed_ast(Exp::Nil))
                                                    ],
                                                    typ: Symbol::from("List"),
                                                }))
                                            ],
                                            typ: Symbol::from("List"),
                                        }))
                                    ],
                                    typ: Symbol::from("List"),
                                }))
                            ],
                            typ: Symbol::from("List"),
                        })
                    ),
                    Pos{line: 0, column: 2}
                )],
            body: boxed_ast(Exp::Var(
                make_var(VarKind::Field(
                    boxed_var(VarKind::Simple(Symbol::from("foo"))),
                    Symbol::from("head")
                ))
            ))
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Function(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    None,
                    boxed_ast(Exp::Unit)
                ),
                Pos{line: 0, column: 0}
            )])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_called_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![
                Dec::Function(vec![(
                    _FunctionDec::new(
                        Symbol::from("foo"),
                        vec![Field {
                            name: Symbol::from("arg1"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }],
                        Some(Symbol::from("int")),
                        boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg1"))))),
                    ),
                    Pos{line: 0, column: 0}
                )]),
                Dec::Function(vec![(
                    _FunctionDec::new(
                        Symbol::from("baaz"),
                        vec![Field {
                            name: Symbol::from("arg2"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }],
                        Some(Symbol::from("int")),
                        boxed_ast(Exp::Call {
                            func: Symbol::from("foo"),
                            args: vec![make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg2")))))],
                        })
                    ),
                    Pos{line: 0, column: 0}
                )]),
            ],
            body: boxed_ast(Exp::Call {
                func: Symbol::from("baaz"),
                args: vec![make_ast(Exp::Int(2))]
            })
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_body_type_error() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Function(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    None,
                    boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("baaz"))))), // undeclared
                ),
                Pos{line: 0, column: 0}
            )])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredSimpleVar(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_body_result_type_mismatch() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Function(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    None,
                    boxed_ast(Exp::Int(2)),
                ),
                Pos{line: 0, column: 0}
            )])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::TypeMismatch(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_repeated_param_names() {
        // TODO
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Function(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    None,
                    boxed_ast(Exp::Unit)
                ),
                Pos{line: 0, column: 0}
            )])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_repeated_function_names() {
        // TODO
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Function(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    None,
                    boxed_ast(Exp::Unit)
                ),
                Pos{line: 0, column: 0}
            )])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_functiondec_recursion() {
        let ast =  make_ast(Exp::Let {
            decs: vec![Dec::Function(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    None,
                    boxed_ast(Exp::Unit)
                ),
                Pos{line: 0, column: 0})])],
            body: boxed_ast(Exp::Unit)
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn letexp_todas_all_decs_ok() {
        let ast =  make_ast(Exp::Let {
            decs: vec![
                Dec::Type(vec![(
                    _TypeDec::new(
                        Symbol::from("FooType"),
                        Ty::Name(Symbol::from("int")),
                    ),
                    Pos{line: 0, column: 0}
                )]),
                Dec::Var(
                    _VarDec::new(
                        Symbol::from("foo"),
                        Some(Symbol::from("FooType")),
                        boxed_ast(Exp::Int(4))
                    ),
                    Pos{line: 0, column: 0}
                ),
                Dec::Function(vec![(
                    _FunctionDec::new(
                        Symbol::from("baaz"),
                        vec![Field {
                            name: Symbol::from("bar"),
                            typ: Ty::Name(Symbol::from("FooType")),
                            escape: false,
                        }],
                        Some(Symbol::from("FooType")),
                        boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("bar")))))
                    ),
                    Pos{line: 0, column: 0}
                )]),
            ],
            body: boxed_ast(Exp::Call {
                func: Symbol::from("baaz"),
                args: vec![make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))]
            })
        });
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }
}