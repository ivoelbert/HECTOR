use super::*;
use frame::WORD_SIZE;

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
            // save results in temp
            // call check_index_array
            // access the memory
            let (array_exp, array_level, array_frags) = trans_var(array, level, value_env, breaks_stack, frags)?;
            let (index_exp, index_level, index_frags) = super::trans_exp(index, array_level, value_env, breaks_stack, array_frags)?;
            let external_label = if let
                EnvEntry::Func {label, ..} = value_env.get("+check_index_array").expect("should be in initial value env")
                { label } else {panic!("typechecking should handle this")};
            Ok((ESEQ(
                Box::new(seq(vec![
                    EXP(Box::new(external_call(external_label.to_string(), vec![array_exp.clone(), index_exp.clone()]))),
                ])),
                Box::new(MEM(Box::new(
                    plus!(
                        array_exp,
                        BINOP(MUL,
                            Box::new(index_exp),
                            Box::new(CONST(WORD_SIZE))
                        )
                    )
                )))
            ), index_level, index_frags))
        },
        VarKind::Field(record, field) => {
            // Similar to array
            // but you use the record's field order as "index"
            let record_typ = record.typ.clone();
            let (record_exp, record_level, record_frags) = trans_var(record, level, value_env, breaks_stack, frags)?;
            let external_label = if let
                EnvEntry::Func {label, ..} = value_env.get("+check_nil").expect("should be in initial value env")
                { label } else {panic!("typechecking should handle this")};
            let formals = if let TigerType::TRecord(formals, ..) = &*record_typ {
                formals.clone()
            } else {
                panic!("not a record. kind: {:?}, typ: {:?}, field: {:?}", kind, typ, field)
            };
            let (_, _, order) = formals
                .iter()
                .find(|(name, ..)| {name == field})
                .expect("typechecking should handle this");
            Ok((ESEQ(
                Box::new(EXP(Box::new(external_call(external_label.to_string(), vec![record_exp.clone()])))),
                Box::new(MEM(Box::new(
                    plus!(
                        record_exp,
                        CONST(*order * WORD_SIZE)
                    )
                )))
            ), record_level, record_frags))
        },
    }
}
