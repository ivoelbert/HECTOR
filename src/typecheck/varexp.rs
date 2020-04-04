use super::*;

fn find_field_type(fields: &[(String, Arc<TigerType>, i32)], symbol: &str) -> Option<Arc<TigerType>> {
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
            let typed_subscript_var = typecheck_var(*subscript_var, type_env, value_env)?;
            let array_of = if let TigerType::TArray(array_of, ..) = &*typed_subscript_var.typ {
                array_of.clone()
            } else {
                return Err(TypeError::NotArrayType(pos))
            };
            let typed_index = type_exp(*index, type_env, value_env)?;
            if *typed_index.typ != TigerType::TInt(R::RW) {
                return Err(TypeError::SubscriptNotInteger(pos))
            };
            Ok(Var{
                kind: VarKind::Subscript(Box::new(typed_subscript_var), Box::new(typed_index)),
                typ: array_of,
                pos
            })
        }
        VarKind::Field(field_var, field_symbol) => {
            // A field var as in:
            // a := foo.bar
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
