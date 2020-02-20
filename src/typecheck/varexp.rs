use super::*;
use crate::utils::log;

fn find_field_type(fields: &[(String, Arc<TigerType>, i64)], symbol: &str) -> Option<Arc<TigerType>> {
    for field in fields {
        if field.0 == symbol {
            return Some(field.1.clone());
        }
    }
    None
}

pub fn typecheck_var(
    Var {kind, pos, ..}: Var,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<Var, TypeError> {
    match kind {
        VarKind::Simple(var_symbol) => match value_env.get(&var_symbol) {
            // A simple var, as in:
            // a := foo
            Some(EnvEntry::Var { ty: var_type, .. }) => Ok(Var {
                kind: VarKind::Simple(var_symbol),
                typ: var_type.clone(),
                pos
            }),
            Some(..) => Err(TypeError::NotSimpleVar(pos)),
            None => Err(TypeError::UndeclaredSimpleVar(pos)),
        },
        VarKind::Subscript(subscript_var, index) => {
            // A subscript var, as in:
            // a := foo[3]
            // foo must be a simple var and the index an integer
            // TODO: Arrays of arrays, records of arrays
            // match *subscript_var {
            //     Var{kind: VarKind::Simple(s), pos, ..} => match value_env.get(&s) {
            //         Some(EnvEntry::Var { ty: array_type, .. }) => match &**array_type {
            //             TigerType::TArray(array_of, _) => {
            //                 let index_ast = type_exp(*index, type_env, value_env)?;
            //                 match *index_ast.typ {
            //                     TigerType::TInt(_) => Ok(Var{
            //                         kind: VarKind::Subscript(
            //                             Box::new(Var{
            //                                 kind: VarKind::Simple(s),
            //                                 pos,
            //                                 typ: array_type.clone()
            //                             }),
            //                             Box::new(index_ast)
            //                         ),
            //                         pos,
            //                         typ: array_of.clone()
            //                     }),
            //                     _ => Err(TypeError::SubscriptNotInteger(pos)),
            //                 }
            //             }
            //             _ => Err(TypeError::NotArrayType(pos)),
            //         },
            //         Some(..) => Err(TypeError::NotArrayType(pos)),
            //         None => Err(TypeError::UndeclaredSimpleVar(pos)),
            //     },
            //     Var{kind: VarKind::Field(..), pos, ..}
            //     | Var{kind: VarKind::Subscript(..), pos, ..} => {
            //         console_log!("NotSimpleVar Array, subscript: {:?}, index: {:?}", subscript_var, index);
            //         Err(TypeError::NotSimpleVar(pos))
            //     },
            // };
            let typed_subscript_var = typecheck_var(*subscript_var, type_env, value_env)?;
            let array_of = if let TigerType::TArray(array_of, ..) = &*typed_subscript_var.typ {
                array_of.clone()
            } else {
                return Err(TypeError::NotArrayType(pos))
            };
            Ok(Var{
                kind: VarKind::Subscript(Box::new(typed_subscript_var), index),
                typ: array_of,
                pos
            })
        }
        VarKind::Field(field_var, field_symbol) => {
            // A field var as in:
            // a := foo.bar
            // match &*field_var {
            //     Var{kind: VarKind::Simple(record_symbol), ..} => match value_env.get(record_symbol) {
            //         Some(env_entry) => match env_entry {
            //             EnvEntry::Var { ty, .. } => match &**ty {
            //                 TigerType::TRecord(vars, _) => match field_type(vars, &field_symbol) {
            //                     Some(field_type) => Ok(Var{
            //                         kind: VarKind::Field(field_var, field_symbol),
            //                         typ: field_type,
            //                         pos
            //                     }),
            //                     None => Err(TypeError::FieldDoesNotExist(pos)),
            //                 },
            //                 _ => Err(TypeError::NotRecordType(pos)),
            //             },
            //             _ => Err(TypeError::NotRecordType(pos)),
            //         },
            //         None => Err(TypeError::UndeclaredSimpleVar(pos)),
            //     },
            //     Var{kind: VarKind::Field(..), ..} | Var{kind: VarKind::Subscript(..), ..} => {
            //         console_log!("NotSimpleVar Record, field_var: {:?}, index: {:?}", field_var, field_symbol);
            //         Err(TypeError::NotSimpleVar(pos))
            //     },
            let typed_field_var = typecheck_var(*field_var, type_env, value_env)?;
            let record_fields = if let TigerType::TRecord(fields, ..) = &*typed_field_var.typ {
                fields
            } else {
                return Err(TypeError::NotRecordType(pos))
            };
            let field_type = if let Some(ty) = find_field_type(&record_fields, &field_symbol) {
                ty
            } else {
                return Err(TypeError::FieldDoesNotExist(pos))
            };
            Ok(Var{
                kind: VarKind::Field(Box::new(typed_field_var), field_symbol),
                typ: field_type,
                pos
            })
        },
    }
}

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    // A Var literal
    match node {
        Exp::Var(var) => {
            let typed_var = typecheck_var(var, type_env, value_env)?;
            let typ = typed_var.typ.clone();
            Ok(AST {
                node: Exp::Var(typed_var),
                pos,
                typ
            })
        },
        _ => panic!("Delegation error varexp::typecheck"),
    }
}
