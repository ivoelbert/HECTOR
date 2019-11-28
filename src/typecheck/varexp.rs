use super::*;

fn field_type(fields: &[(String, Arc<TigerType>, i64)], symbol: &str) -> Option<Arc<TigerType>> {
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
            match *subscript_var {
                Var{kind: VarKind::Simple(s), pos, ..} => match value_env.get(&s) {
                    Some(EnvEntry::Var { ty: array_type, .. }) => match &**array_type {
                        TigerType::TArray(array_of, _) => {
                            let index_ast = type_exp(*index, type_env, value_env)?;
                            match *index_ast.typ {
                                TigerType::TInt(_) => Ok(Var{
                                    kind: VarKind::Subscript(
                                        Box::new(Var{
                                            kind: VarKind::Simple(s),
                                            pos,
                                            typ: array_type.clone()
                                        }),
                                        Box::new(index_ast)
                                    ),
                                    pos,
                                    typ: array_of.clone()
                                }),
                                _ => Err(TypeError::SubscriptNotInteger(pos)),
                            }
                        }
                        _ => Err(TypeError::NotArrayType(pos)),
                    },
                    Some(..) => Err(TypeError::NotArrayType(pos)),
                    None => Err(TypeError::UndeclaredSimpleVar(pos)),
                },
                Var{kind: VarKind::Field(..), pos, ..}
                | Var{kind: VarKind::Subscript(..), pos, ..} => Err(TypeError::NotSimpleVar(pos)),
            }
        }
        VarKind::Field(subscript_var, field_symbol) =>
            // A field var as in:
            // a := foo.bar
            match &*subscript_var {
                Var{kind: VarKind::Simple(record_symbol), ..} => match value_env.get(record_symbol) {
                    Some(env_entry) => match env_entry {
                        EnvEntry::Var { ty, .. } => match &**ty {
                            TigerType::TRecord(vars, _) => match field_type(vars, &field_symbol) {
                                Some(field_type) => Ok(Var{
                                    kind: VarKind::Field(subscript_var, field_symbol),
                                    typ: field_type,
                                    pos
                                }),
                                None => Err(TypeError::FieldDoesNotExist(pos)),
                            },
                            _ => Err(TypeError::NotRecordType(pos)),
                        },
                        _ => Err(TypeError::NotRecordType(pos)),
                    },
                    None => Err(TypeError::UndeclaredSimpleVar(pos)),
                },
                Var{kind: VarKind::Field(..), ..} | Var{kind: VarKind::Subscript(..), ..} => Err(TypeError::NotSimpleVar(pos)),
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
        _ => panic!("le llego algo nada que ver a varexp::tipar"),
    }
}
