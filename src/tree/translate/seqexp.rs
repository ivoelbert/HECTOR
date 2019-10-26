use crate::ast::*;
use crate::tree::*;

fn trans_seq(
    exps: &[Exp],
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    exps.iter().try_fold(
        (EXP(Box::new(CONST(0))), frags),
        |(prev_tree, frags), exp| -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
            let (tree, frags) = super::trans_stm(
                exp,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                frags,
            )?;
            Ok((SEQ(Box::new(prev_tree), Box::new(tree)), frags))
        },
    )
}

pub fn trans_exp<'a>(
    Exp { node, .. }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Seq(exps) => match exps.split_last() {
            Some((last, rest)) => {
                let (prev_stms, prev_frags) = trans_seq(
                    rest,
                    levels.clone(),
                    value_env.clone(),
                    breaks_stack.clone(),
                    frags,
                )?;
                let (last, last_frags) =
                    super::trans_exp(last, levels, value_env, breaks_stack, prev_frags)?;
                Ok((ESEQ(Box::new(prev_stms), Box::new(last)), last_frags))
            }
            None => panic!("empty seq"),
        },
        _ => panic!(),
    }
}

pub fn trans_stm<'a>(
    Exp { node, .. }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::Seq(exps) => match exps.split_last() {
            Some((last, rest)) => {
                let (prev_stms, seq_frags) = trans_seq(
                    rest,
                    levels.clone(),
                    value_env.clone(),
                    breaks_stack.clone(),
                    frags,
                )?;
                let (last, last_frags) =
                    trans_stm(last, levels, value_env, breaks_stack, seq_frags)?;
                Ok((SEQ(Box::new(prev_stms), Box::new(last)), last_frags))
            }
            None => panic!("empty seq"),
        },
        _ => panic!(),
    }
}
