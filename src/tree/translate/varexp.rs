use super::*;

// Generates an expression that evaluates to the memory direction of the variable
pub fn simplevar(access: Access, current_level: &Level, depth: i32) -> Tree::Exp {
    current_level.access_to_exp(access, depth)
}

pub fn trans_var(
    Var{kind, typ, ..}: &Var,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match kind {
        VarKind::Simple(name) => {
            if let Some(EnvEntry::Var{access, depth}) = value_env.get(name) {
                Ok((
                    simplevar(access.clone(), &level, *depth),
                    level,
                    frags
                ))
            } else {
                panic!("var not in env. value_env: {:?}, name {:?}", value_env, name)
            }
        },
        VarKind::Subscript(array, index) => {
            // The array is the memory pointer to the start
            // sum it with index times data size
            // there's a runtime to fail nicely
            // or we can let the dev get a segfault
            let (array_exp, array_level, array_frags) = trans_var(array, level, value_env, breaks_stack, frags)?;
            let (index_exp, index_level, index_frags) = super::trans_exp(index, array_level, value_env, breaks_stack, array_frags)?;
            Ok((MEM(Box::new(plus!(
                array_exp,
                index_exp
            ))), index_level, index_frags))
        },
        VarKind::Field(record, field) => {
            // Similar to array
            // but you use the record's field order as "index"
            let record_typ = record.typ.clone();
            let (record_exp, record_level, record_frags) = trans_var(record, level, value_env, breaks_stack, frags)?;
            let formals = if let TigerType::TRecord(formals, ..) = &*record_typ {
                formals.clone()
            } else {
                panic!("not a record. kind: {:?}, typ: {:?}, field: {:?}", kind, typ, field)
            };
            let (_, _, order) = formals
                .iter()
                .find(|(name, ..)| {name == field})
                .expect("typechecking should handle this");
            Ok((MEM(Box::new(plus!(
                record_exp,
                // optimization candidate
                CONST(*order)
            ))), record_level, record_frags))
        },
    }
}
