use crate::ast::*;
use crate::tree::*;

mod arrayexp;
mod assignexp;
mod breakexp;
mod callexp;
mod forexp;
mod ifexp;
mod intexp;
mod letexp;
mod nilexp;
mod opexp;
mod recordexp;
mod seqexp;
mod stringexp;
mod unitexp;
mod varexp;
mod whileexp;

pub fn trans_exp(
    exp: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match exp {
        Exp { node, .. } => match node {
            _Exp::Var(var) => varexp::trans_var(var, value_env, breaks_stack, prev_frags),
            _Exp::Unit => unitexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Nil => nilexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Int(_) => intexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::String(_) => stringexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Call { .. } => callexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Op { .. } => opexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Record { .. } => recordexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::If { .. } => ifexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Let { .. } => letexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Array { .. } => arrayexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _Exp::Seq(_) => seqexp::trans_exp(exp, value_env, breaks_stack, prev_frags),
            _ => panic!("cannot translate as exp!")
        },
    }
}

fn trans_stm(
    exp: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Vec<Fragment>), TransError> {
    match exp {
        Exp { node, .. } => match node {
            _Exp::Break => breakexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _Exp::Call { .. } => callexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _Exp::Assign { .. } => assignexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _Exp::Seq(_) => seqexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _Exp::If { .. } => ifexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _Exp::While { .. } => whileexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _Exp::For { .. } => forexp::trans_stm(exp, value_env, breaks_stack, prev_frags),
            _ => {
                let (exp, frags) = trans_exp(exp, value_env, breaks_stack, prev_frags)?;
                Ok((Tree::Stm::EXP(Box::new(exp)), frags))
            }
        },
    }
}

// TODO: nuke this
fn trans_cond(
    exp: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Cond, Vec<Fragment>), TransError> {
    match exp {
        Exp { node, .. } => match node {
            _Exp::Op { .. } => opexp::trans_cond(exp, value_env, breaks_stack, prev_frags),
            _ => panic!("cannot translate as cond!")
        },
    }
}

pub fn tranlate(exp: Exp) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    let tiger_main = boxed_exp(_Exp::Let {
            decs: vec![Dec::FunctionDec(vec![(_FunctionDec{
                name: Label::from("_tigermain"),
                params: vec![],
                body: Box::new(exp),
                result: None,
            }, Pos{line: 0, column: 0})])],
            body: boxed_exp(_Exp::Unit)
        });
    trans_exp(&tiger_main, &initial_value_env(), vec![], vec![])
}