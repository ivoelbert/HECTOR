use super::super::ast::tigerabs::*;
use super::super::ast::position::Pos;
use super::tigerseman::*;


fn tipar_decs(decs: Vec<Dec>, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<(TypeEnviroment, ValueEnviroment), TypeError> {
    use Dec::*;
    fn sort_decs(decs: Vec<Dec>) -> Vec<Dec> {
        // Esto eventualmente debería hacer un sort topologico.
        // Encuentra ciclos.
        decs
    }

    fn tipar_dec_variable(_VarDec {name, typ, init, ..}: _VarDec, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
        let mut new_value_env = value_env.clone();
        let init_type = tipar_exp(*init, type_env, value_env)?;
        let dec_type = match typ {
            None => init_type,
            Some(typ_string) => match type_env.get(&typ_string) {
                Some(table_type) =>
                    if *table_type == init_type {
                        table_type.clone()
                    } else {
                        return Err(TypeError::TypeMismatch(pos))
                    },
                None => return Err(TypeError::UndeclaredType(pos))
            }
        };
        new_value_env.insert(name, EnvEntry::Var {
            ty: dec_type,
            access: Access::InFrame(0),
            level: 0
        });
        Ok(new_value_env)
    }

    fn tipar_decs_bloque_funciones(decs: Vec<_FunctionDec>, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<ValueEnviroment, TypeError> {
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
        // for fun in decs {
            
        // }
        // // Si hay results, fijarse que coincidan los tipos.
        // // Devolver los envs con los prototipos.
        Ok(new_value_env)
    }

    fn tipar_decs_bloque_tipos(decs: Vec<_TypeDec>, type_env: &TypeEnviroment, value_env: &ValueEnviroment, pos: Pos) -> Result<TypeEnviroment, TypeError> {
        let mut new_type_env = type_env.clone();
        Ok(new_type_env)
    }

    let sorted_decs = sort_decs(decs);
    let mut new_type_env = type_env.clone();
    let mut new_value_env = value_env.clone();
    // for dec in sorted_decs {
    //     let (new_type_env, new_value_env) = match dec {
    //         VarDec(vd) => match tipar_dec_variable(vd, &new_type_env, &new_value_env, pos) {
    //             Ok(venv) => (new_type_env, venv),
    //             Err(type_error) => return Err(type_error),
    //         },
    //         FunctionDec(fd) => match tipar_decs_bloque_funciones(fd, &new_type_env, &new_value_env, pos) {
    //             Ok(venv) => (new_type_env, venv),
    //             Err(type_error) => return Err(type_error),
    //         },
    //         TypeDec(td) => match tipar_decs_bloque_tipos(td, &new_type_env, &new_value_env, pos) {
    //             Ok(tenv) => (tenv, new_value_env),
    //             Err(type_error) => return Err(type_error),
    //         },
    //     };
        
    // }
    Ok((new_type_env, new_value_env))
}

pub fn tipar(exp: Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp {
        Exp {node: _Exp::LetExp {decs, body}, pos} => {
            let (new_type_env, new_value_env) =  tipar_decs(decs, type_env, value_env, pos)?;
            tipar_exp(*body, &new_type_env, &new_value_env)
        },
        _ => panic!("error de delegacion en letexp::tipar"),
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}