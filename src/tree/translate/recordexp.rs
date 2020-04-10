use crate::ast::*;
use crate::tree::*;

// fn gen_init_stm(memaddr: Tree::Exp, typ: Arc<TigerType>, mut fields: HashMap<String, Tree::Exp>) -> Tree::Stm {
//     if let TigerType::TRecord(formals, ..) = &*typ {
//         seq(formals
//             .iter()
//             .map(|(name, _, order)| {
//                 Move!(
//                     MEM(Box::new(plus!(
//                         memaddr.clone(),
//                         CONST(*order * frame::WORD_SIZE)
//                     ))),
//                     fields.remove(name).expect("typechecking should handle this")
//                 )
//             })
//             .collect()
//         )
//     } else {
//         panic!("typechecking should handle this")
//     }
// }

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    // Call an external to alloc dynamic memory.
    // Move data to that memory.
    match node {
        Exp::Record { fields, .. } => {
            use std::convert::TryInto;
            // Translate field initializations
            let (fields_exps, fields_level, fields_frags): (Vec<Tree::Exp>, Level, Vec<Fragment>) = fields
                .iter()
                .try_fold((vec![], level, frags), |(mut exps, level, frags), (_name, ast)| {
                    let (exp, level, frags) = super::trans_exp(ast, level, value_env, breaks_stack, frags)?;
                    exps.push(exp);
                    Ok((exps, level, frags))
                })?;
            if let EnvEntry::Func {label, ..} = value_env.get("+alloc_record").expect("should be in initial value env") {
                Ok((
                    // This returns the memory address of the malloc result
                    external_call(label.clone(), vec![vec![CONST(fields_exps.len().try_into().unwrap())], fields_exps].concat()),
                    fields_level, fields_frags
                ))
            }
            else {
                panic!("external function not found");
            }
        }
        _ => panic!(),
    }
}