use super::super::ast::tigerabs::*;
use super::super::ast::position::Pos;
use super::tigerseman::*;

fn tipar_dec_variable(_VarDec {name, typ, init, ..}: &_VarDec, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    let mut new_value_env = value_env.clone();
    let init_type = tipar_exp(init, type_env, value_env)?;
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
    new_value_env.insert(name.clone(), EnvEntry::Var {
        ty: dec_type,
        access: Access::InFrame(0),
        level: 0
    });
    Ok(new_value_env)
}

fn tipar_dec_funcion(_FunctionDec {name, params, result, body, pos}: &_FunctionDec, mut value_env: ValueEnviroment, type_env: &TypeEnviroment) -> Result<ValueEnviroment, TypeError> {
    let tipar_ty = |ty: &Ty| -> Result<Tipo, TypeError> {
            match ty {
                Ty::Name(symbol) => match type_env.get(symbol) {
                    Some(tipo) => Ok(tipo.clone()),
                    None => Err(TypeError::UndeclaredType(*pos))
                },
                _ => panic!("Solo pueden llegar NameTys a parametros de funciones")
            }
    };
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
                ty: tipar_ty(typ)?,
                access: Access::InReg(name.clone()),
                level: 0
            });
            Ok(prev)
        })?;
    let formals: Vec<Tipo> = params
        .iter()
        .map(|Field {typ, ..}: &Field| -> Result<Tipo, TypeError> {tipar_ty(typ)})
        .collect::<Result<Vec<Tipo>, TypeError>>()?;

    // Tipar el body
    let body_type = tipar(&*body, type_env, &params_value_env)?;
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

fn tipar_decs_bloque_funciones(decs: &Vec<_FunctionDec>, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
    fn sort_funcs(decs: Vec<_FunctionDec>) -> Vec<_FunctionDec> {
        // Esto eventualmente debería hacer un sort topologico.
        // Encuentra ciclos.
        decs
    }

    let sorted_decs = decs;
    // Checkear que no haya funciones repetidas.

    // Checkear que ninguna funcion tenga parametros repetidos.

    // Armar envs con los prototipos.
    let mut new_value_env = value_env.clone();
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
    for dec in sorted_decs {
        new_value_env = tipar_dec_funcion(dec, new_value_env, type_env)?;
    }
    Ok(new_value_env)
    // // Si hay results, fijarse que coincidan los tipos.
    // // Devolver los envs con los prototipos.
}

fn tipar_decs_bloque_tipos(decs: &Vec<_TypeDec>, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<TypeEnviroment, TypeError> {
    let mut new_type_env = type_env.clone();
    Ok(new_type_env)
}

fn tipar_decs(decs: &Vec<Dec>, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<(TypeEnviroment, ValueEnviroment), TypeError> {


    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    for dec in decs {
        match dec {
            Dec::VarDec(vd) => match tipar_dec_variable(vd, &new_type_env, &new_value_env, pos) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::FunctionDec(fd) => match tipar_decs_bloque_funciones(fd, &new_type_env, &new_value_env, pos) {
                Ok(venv) => new_value_env = venv,
                Err(type_error) => return Err(type_error),
            },
            Dec::TypeDec(td) => match tipar_decs_bloque_tipos(td, &new_type_env, &new_value_env, pos) {
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