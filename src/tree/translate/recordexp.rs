use crate::ast::*;
use crate::tree::*;

pub use std::sync::Arc;
use crate::typecheck::TigerType;

pub fn trans_exp(
    AST {node, typ, ..}: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    // Call an external to alloc dynamic memory.
    // Move data to that memory.
    match node {
        Exp::Record { fields, .. } => {
            use std::convert::TryInto;
            // Translate field initializations
            let (mut fields_exps, mut fields_level, fields_frags) = fields
                .iter()
                .try_fold((HashMap::new(), level, frags), |(mut exps, level, frags), (name, ast)| {
                    let (exp, level, frags) = super::trans_exp(ast, level, value_env, breaks_stack, frags)?;
                    exps.insert(name.clone(), exp);
                    Ok((exps, level, frags))
            })?;
            let record_size = fields.iter().count();
            let memory_address = fields_level.alloc_local(false, None);
            let label = if let
                EnvEntry::Func {label, ..} = value_env.get("+alloc_record").expect("should be in initial value env")
                { label } else {panic!("typechecking should handle this")};
            let formals = if let
                TigerType::TRecord(formals, ..) = &**typ
                { formals } else {panic!("typechecking should handle this")};
            let init_statement = seq(formals
                .iter()
                .map(|(name, _, order)| {
                    Move!(
                        MEM(Box::new(plus!(
                            fields_level.access_to_exp(memory_address.clone(), fields_level.nesting_depth),
                            CONST(*order * WORD_SIZE)
                        ))),
                        fields_exps.remove(name).expect("typechecking should handle this")
                    )
                })
                .collect()
            );
            Ok((
                // Alloc the record memory, store memory address in a temp.
                // copy the inits
                // return the temp with the address
                ESEQ(
                    Box::new(seq(vec![
                        Move!(
                            fields_level.access_to_exp(memory_address.clone(), fields_level.nesting_depth),// This returns the memory address of the malloc result
                            external_call(label.to_string(), vec![CONST(record_size.try_into().unwrap())])
                        ),
                        init_statement
                    ])),
                    Box::new(fields_level.access_to_exp(memory_address, fields_level.nesting_depth)),
                ),
                fields_level, fields_frags
            ))
        }
        _ => panic!(),
    }
}