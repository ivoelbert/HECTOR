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
    // Selects the correct runtime function to handle a string operation
    // Translate to a NAME of that function.
    use Oper::*;
    let external_name = match ast_oper {
        PlusOp | MinusOp | TimesOp | DivideOp => {
            panic!("Not a string operation.")
        },
        EqOp => "+str_equals",
        NeqOp => "+str_not_equals",
        LtOp => "+str_lower",
        LeOp => "+str_lower_or_equals",
        GtOp => "+str_greater",
        GeOp => "+str_greater_or_equals",
    };
    let entry = value_env.get(external_name);
    if let Some(EnvEntry::Func {label, external: true}) = entry {
        Box::new(NAME(label.clone()))
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