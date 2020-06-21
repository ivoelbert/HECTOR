use super::*;

fn trans_int_oper(ast_oper: Oper) -> Tree::BinOp {
    match ast_oper {
        Oper::Plus => PLUS,
        Oper::Minus => MINUS,
        Oper::Times => MUL,
        Oper::Divide => DIV,
        Oper::Eq => EQ,
        Oper::Neq => NE,
        Oper::Lt => ULT, // LT?
        Oper::Le => ULE,
        Oper::Gt => UGT,
        Oper::Ge => UGE,
    }
}

fn trans_str_oper(ast_oper: Oper, value_env: &ValueEnviroment) -> Box<Tree::Exp> {
    // Selects the correct runtime function to handle a string operation
    // Translate to a NAME of that function.
    let external_name = match ast_oper {
        Oper::Plus | Oper::Minus | Oper::Times | Oper::Divide => {
            panic!("Not a string operation.")
        },
        Oper::Eq => "+str_equals",
        Oper::Neq => "+str_not_equals",
        Oper::Lt => "+str_less",
        Oper::Le => "+str_less_or_equals",
        Oper::Gt => "+str_greater",
        Oper::Ge => "+str_greater_or_equals",
    };
    let entry = value_env.get(external_name);
    if let Some(EnvEntry::Func {label, external: true, ..}) = entry {
        Box::new(NAME(label.clone()))
    } else {
        panic!("should be in initial value env")
    }
}

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Op{left, right, oper} => {
            let (left_exp, left_level, left_frags) = super::trans_exp(left, level, value_env, breaks_stack, frags)?;
            let (right_exp, right_level, right_frags) = super::trans_exp(right, left_level, value_env, breaks_stack, left_frags)?;
            match *left.typ {
                // TigerType::TInt(_) => Ok((
                //     BINOP(trans_int_oper(*oper), Box::new(left_exp), Box::new(right_exp)),
                //     right_level,
                //     right_frags
                // )),
                TigerType::TString => {
                    let proc_label = trans_str_oper(*oper, value_env);
                    Ok((
                        CALL(proc_label, vec![left_exp, right_exp]),
                        right_level,
                        right_frags
                    ))
                },
                // TigerType::TNil => Ok((
                //     CONST(0),
                //     right_level,
                //     right_frags
                // )),
                // TigerType::TRecord(..) =>
                    // if let TigerType::TNil = *right.typ {
                    //     Ok((
                    //         CONST(0),
                    //         right_level,
                    //         right_frags
                    //     ))
                    // } else {
                    // },
                _ => {
                    Ok((
                        BINOP(trans_int_oper(*oper), Box::new(left_exp), Box::new(right_exp)),
                        right_level,
                        right_frags
                    ))
                }
            }
        }
        _ => panic!("delegation error")
    }
}