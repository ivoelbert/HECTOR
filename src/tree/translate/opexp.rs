use crate::ast::*;
use crate::tree::*;

fn trans_int_oper(ast_oper: &Oper) -> Tree::BinOp {
    use Oper::*;
    match ast_oper {
        PlusOp => PLUS,
        MinusOp => MINUS,
        TimesOp => MUL,
        DivideOp => DIV,
        EqOp => EQ,
        NeqOp => NE,
        LtOp => ULT, // LT?
        LeOp => ULE,
        GtOp => UGT,
        GeOp => UGE,
    }
}

fn trans_str_oper(ast_oper: &Oper, value_env: &ValueEnviroment) -> Box<Tree::Exp> {
    use Oper::*;
    // TODO
    // use names imposible to form to avoid overloading without typechecking
    let entry = value_env.get(&String::from("unimplemented"));
    if let Some(EnvEntry::Func {label, external: true}) = entry {
        Box::new(NAME(*label))
    } else {
        panic!("should be in initial value env")
    }
}

pub fn trans_exp(
    AST {node, ..}: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Op{left, right, oper} => {
            let (left_exp, left_level, left_frags) = super::trans_exp(left, level, value_env, breaks_stack, frags)?;
            let (right_exp, right_level, right_frags) = super::trans_exp(right, left_level, value_env, breaks_stack, left_frags)?;
            match *left.typ {
                TigerType::TInt(_) => Ok((
                    BINOP(trans_int_oper(oper), Box::new(left_exp), Box::new(right_exp)),
                    right_level,
                    right_frags
                )),
                TigerType::TString => Ok((
                    CALL(trans_str_oper(oper, value_env), vec![left_exp, right_exp]),
                    right_level,
                    right_frags
                )),
                _ => panic!("typechecking should not allow this")
            }
        }
        _ => panic!("delegation error")
    }
}