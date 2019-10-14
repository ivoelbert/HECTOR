use crate::ast::*;
use crate::tree::*;

fn trans_seq(
    exps: &[Exp],
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    let  (statements, fragments) : (Vec<Tree::Stm>, Vec<Vec<Fragment>>) = exps
        .iter()
        // This could be much faster if we avoided the clonings
        .map(|exp| trans_stm(exp, value_env, breaks_stack.clone(), prev_frags.clone()))
        .collect::<Result<Vec<(Tree::Stm, Vec<Fragment>)>, TransError>>()?
        .iter()
        .cloned()
        .unzip();
    Ok((seq(statements), fragments.concat()))
}

pub fn trans_exp(
    Exp { node, .. }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Seq(exps) => {
            match exps.split_last() {
                Some((last, rest)) => {
                    let (prev_stms, seq_frags) = trans_seq(rest, value_env, breaks_stack.clone(), prev_frags)?;
                    let (last, last_frags) = trans_exp(last, value_env, breaks_stack, seq_frags)?;
                    Ok((ESEQ(Box::new(prev_stms), Box::new(last)), last_frags))
                }
                None => panic!("empty seq")
            }
        },
        _ => panic!()
    }
}

pub fn trans_stm(
    Exp { node, .. }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match node {
        _Exp::Seq(exps) => {
            match exps.split_last() {
                Some((last, rest)) => {
                    let (prev_stms, seq_frags) = trans_seq(rest, value_env, breaks_stack.clone(), prev_frags)?;
                    let (last, last_frags) = trans_stm(last, value_env, breaks_stack, seq_frags)?;
                    Ok((SEQ(Box::new(prev_stms), Box::new(last)), last_frags))
                }
                None => panic!("empty seq")
            }
        },
        _ => panic!()
    }
}