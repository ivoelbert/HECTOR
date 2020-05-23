use crate::ast::*;
use crate::tree::*;

fn trans_seq(
    exps: &[AST],
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    exps.iter().try_fold(
        (EXP(Box::new(CONST(0))), level, frags),
        |(prev_tree, level, frags), exp| -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
            let (tree, level, frags) = super::trans_stm(exp, level, value_env, breaks_stack, frags)?;
            Ok((SEQ(Box::new(prev_tree), Box::new(tree)), level, frags))
        },
    )
}

pub fn trans_exp(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Seq(exps) => match exps.split_last() {
            Some((last, rest)) => {
                let (prev_stms, prev_level, prev_frags) =
                    trans_seq(rest, level, value_env, breaks_stack, frags)?;
                let (last, last_level, last_frags) =
                    super::trans_exp(last, prev_level, value_env, breaks_stack, prev_frags)?;
                Ok((ESEQ(Box::new(prev_stms), Box::new(last)), last_level, last_frags))
            }
            None => panic!("empty seq"),
        },
        _ => panic!(),
    }
}

pub fn trans_stm(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::Seq(exps) => match exps.split_last() {
            Some((last, rest)) => {
                let (prev_stms, prev_level, seq_frags) =
                    trans_seq(rest, level, value_env, breaks_stack, frags)?;
                let (last, last_level, last_frags) =
                    super::trans_stm(last, prev_level, value_env, breaks_stack, seq_frags)?;
                Ok((SEQ(Box::new(prev_stms), Box::new(last)), last_level, last_frags))
            }
            None => panic!("empty seq"),
        },
        _ => {
            panic!("delegation error")
        },
    }
}
