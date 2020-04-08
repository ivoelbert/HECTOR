use crate::ast::*;
use crate::tree::*;

fn gen_init_stm(memaddr: Tree::Exp, typ: Arc<TigerType>, mut fields: HashMap<String, Tree::Exp>) -> Tree::Stm {
    if let TigerType::TRecord(formals, ..) = &*typ {
        seq(formals
            .iter()
            .map(|(name, _, order)| {
                Move!(
                    MEM(Box::new(plus!(
                        memaddr.clone(),
                        CONST(*order * frame::WORD_SIZE)
                    ))),
                    fields.remove(name).expect("typechecking should handle this")
                )
            })
            .collect()
        )
    } else {
        panic!("typechecking should handle this")
    }
}

pub fn trans_exp(
    AST {node, typ, ..}: &AST,
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
            let (fields_exps, mut fields_level, fields_frags) = fields
                .iter()
                .try_fold((HashMap::new(), level, frags), |(mut exps, level, frags), (name, ast)| {
                    let (exp, level, frags) = super::trans_exp(ast, level, value_env, breaks_stack, frags)?;
                    exps.insert(name.clone(), exp);
                    Ok((exps, level, frags))
                })?;
            let memaddr_access = fields_level.alloc_local(false, None);
            let memaddr_exp = fields_level.access_to_exp(memaddr_access.clone());
            if let EnvEntry::Func {label, ..} = value_env.get("+alloc_record").expect("should be in initial value env") {
                Ok((
                    ESEQ(Box::new(SEQ(
                        Box::new(Move!(
                            memaddr_exp.clone(),
                            // This returns the memory address of the malloc result
                            external_call(label.clone(), vec![CONST(fields_exps.len().try_into().unwrap())])
                        )),
                        Box::new(gen_init_stm(memaddr_exp.clone(), typ.clone(), fields_exps))
                        )),
                        Box::new(memaddr_exp.clone())
                    ),
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