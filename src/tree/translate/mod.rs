use crate::ast::*;
use crate::tree::*;
use crate::utils::log;

mod arrayexp;
mod assignexp;
pub mod breakexp;
mod callexp;
mod forexp;
mod ifexp;
mod intexp;
mod letexp;
mod nilexp;
mod opexp;
mod recordexp;
mod seqexp;
pub mod stringexp;
mod unitexp;
mod varexp;
mod whileexp;

// Translation functions.
// We replaced all side-effects in Appel's book for move semantics because it's our compiler.
// Also, no packing and unpacking. No conditionals either, only expressions and statements.
fn trans_exp(
    exp: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    // console_log!("exp: {:?}", &exp);
    match exp {
        AST { node, .. } => match node {
            Exp::Var(var) => varexp::trans_var(var, level, value_env, breaks_stack, prev_frags),
            Exp::Unit => unitexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Nil => nilexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Int(_) => intexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::String(_) => stringexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Call { .. } => callexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Op { .. } => opexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Record { .. } => recordexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::If { .. } => ifexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Let { .. } => letexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Array { .. } => arrayexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            Exp::Seq(_) => seqexp::trans_exp(exp, level, value_env, breaks_stack, prev_frags),
            _ => panic!("cannot translate as exp!")
        },
    }
}

fn trans_stm(
    stm: &AST,
    levels: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    // console_log!("stm: {:?}", &stm);
    match stm {
        AST { node, .. } => match node {
            Exp::Break => breakexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            Exp::Call { .. } => callexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            Exp::Assign { .. } => assignexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            Exp::Seq(_) => seqexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            Exp::If { .. } => ifexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            Exp::While { .. } => whileexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            Exp::For { .. } => forexp::trans_stm(stm, levels, value_env, breaks_stack, prev_frags),
            _ => {
                let (exp, level, frags) = trans_exp(stm, levels, value_env, breaks_stack, prev_frags)?;
                Ok((Tree::Stm::EXP(Box::new(exp)), level, frags))
            }
        },
    }
}

fn translate_many_exp(
    exps: &[AST],
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

pub fn translate(exp: AST) -> Result<(Vec<Fragment>), TransError> {
    let level = Level::outermost();
    console_log!("Outermost");
    let value_env = initial_value_env();
    console_log!("Value Env");
    let tiger_main = make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(_FunctionDec{
            name: String::from("_tigermain"),
            params: vec![],
            body: Box::new(exp),
            result: None,
        }, Pos{line: 0, column: 0})])],
        body: boxed_ast(Exp::Unit)
    });
    console_log!("Arrancando traduccion.");
    let (_, _, main_frags) = trans_exp(&tiger_main, level, &value_env, &vec![], vec![])?;
    return Ok(main_frags);
}