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

// Translation functions.
// We replaced all side-effects in Appel's book for move semantics because it's our compiler.
// Also, no packing and unpacking. No conditionals either, only expressions and statements.
fn trans_exp(
    exp: &Exp,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match exp {
        Exp { node, .. } => match node {
            _Exp::Var(var) => varexp::trans_var(var, level, value_env, breaks_stack, prev_frags),
            _Exp::Unit => unitexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Nil => nilexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Int(_) => intexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::String(_) => stringexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Call { .. } => callexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Op { .. } => opexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Record { .. } => recordexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::If { .. } => ifexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Let { .. } => letexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Array { .. } => arrayexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _Exp::Seq(_) => seqexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _ => panic!("cannot translate as exp!")
        },
    }
}

fn trans_stm(
    exp: &Exp,
    levels: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match exp {
        Exp { node, .. } => match node {
            _Exp::Break => breakexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _Exp::Call { .. } => callexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _Exp::Assign { .. } => assignexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _Exp::Seq(_) => seqexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _Exp::If { .. } => ifexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _Exp::While { .. } => whileexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _Exp::For { .. } => forexp::trans_stm(exp, levels, value_env, breaks_stack, prev_frags),
            _ => {
                let (exp, level, frags) = trans_exp(exp, levels, value_env, breaks_stack, prev_frags)?;
                Ok((Tree::Stm::EXP(Box::new(exp)), level, frags))
            }
        },
    }
}

fn translate_many_exp(
    exps: &[Exp],
    mut level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    mut frags: Vec<Fragment>,
) -> Result<(Vec<Tree::Exp>, Level, Vec<Fragment>), TransError> {
    let mut interm_exps : Vec<Tree::Exp> = vec![];
    for exp in exps {
        let (i, l, f) = trans_exp(exp, level, value_env, breaks_stack, frags)?;
        level = l;
        interm_exps.push(i);
        frags = f;
    }
    Ok((interm_exps, level, frags))
}

pub fn translate(exp: &Exp) -> Result<(Vec<Fragment>), TransError> {
    // let tiger_main = boxed_exp(_Exp::Let {
    //         decs: vec![Dec::FunctionDec(vec![(_FunctionDec{
    //             name: Label::from("_tigermain"),
    //             params: vec![],
    //             body: Box::new(exp),
    //             result: None,
    //         }, Pos{line: 0, column: 0})])],
    //         body: boxed_exp(_Exp::Unit)
    //     });
    let level = Level::outermost();
    let value_env = initial_value_env();
    Ok(trans_exp(exp, level.clone(), &value_env, &vec![], vec![])?.2)
}