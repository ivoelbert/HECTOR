use crate::ast::*;
use crate::tree::*;

fn gen_init_stm(temp: Temp, typ: Arc<TigerType>, mut fields: HashMap<String, Tree::Exp>) -> Tree::Stm {
    // Todo esto no hace falta, al runtime le podemos pasar la lista de parametros para inicializar (ordenada, supongo) y ya los mete.
    // TODO
    if let TigerType::TRecord(formals, ..) = &*typ {
        // formals.sort_by(|(_, _, order_a), (_, _, order_b)| order_a.partial_cmp(order_b).unwrap());
        seq(formals
            .iter()
            .map(|(name, _, order)| {
                Move!(
                    MEM(Box::new(plus!(
                        TEMP(temp.clone()),
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
            let (fields_exps, fields_level, fields_frags) = fields
                .iter()
                .try_fold((HashMap::new(), level, frags), |(mut exps, level, frags), (name, ast)| {
                    let (exp, level, frags) = super::trans_exp(ast, level, value_env, breaks_stack, frags)?;
                    exps.insert(name.clone(), exp);
                    Ok((exps, level, frags))
                })?;
            let temp = newtemp();
            // use a name not posible in Tiger
            if let EnvEntry::Func {label, ..} = value_env.get("+alloc_record").expect("should be in initial value env") {
                Ok((
                    ESEQ(
                        Box::new(SEQ(
                            Box::new(Move!(
                                TEMP(temp.clone()),
                                // This returns the memory address of the malloc result
                                Frame::external_call(String::from("+alloc_record"), label.clone(), vec![CONST(fields_exps.len().try_into().unwrap())])
                            )),
                            Box::new(gen_init_stm(temp.clone(), typ.clone(), fields_exps))
                        )),
                        Box::new(TEMP(temp.clone()))
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