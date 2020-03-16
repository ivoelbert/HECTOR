use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::If {
            test,
            then_,
            else_: Some(else_),
        } => {
            let (test_exp, test_level, test_frags) = super::trans_exp(
                &**test,
                level,
                value_env,
                breaks_stack,
                frags,
            )?;
            let (then_exp, then_level, then_frags) = super::trans_exp(
                then_,
                test_level,
                value_env,
                breaks_stack,
                test_frags,
            )?;
            let (else_exp, else_level, else_frags) =
                super::trans_exp(else_, then_level, value_env, breaks_stack, then_frags)?;
            let (then_label, join_label, else_label) = (newlabel(), newlabel(), newlabel());
            let result = newtemp();
            let compare_exp = BINOP(GE, Box::new(test_exp), Box::new(CONST(1)));
            Ok((
                ESEQ(
                    Box::new(Tree::seq(vec![
                        CJUMP(
                            compare_exp,
                            then_label.clone(),
                            else_label.clone(),
                        ),
                        LABEL(then_label.clone()),
                        Move!(TEMP(result.clone()), then_exp),
                        JUMP(NAME(join_label.clone()), vec![join_label.clone()]),
                        LABEL(else_label.clone()),
                        Move!(TEMP(result.clone()), else_exp),
                        LABEL(join_label.clone()),
                    ])),
                    Box::new(TEMP(result.clone())),
                ),
                else_level,
                else_frags,
            ))
        }
        _ => panic!("not an if"),
    }
}

pub fn trans_stm(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Stm, Level, Vec<Fragment>), TransError> {
    match node {
        Exp::If {
            test,
            then_,
            else_: None,
        } => {
            let (test_exp, test_level, test_frags) = super::trans_exp(
                &**test,
                level,
                value_env,
                breaks_stack,
                frags,
            )?;
            let (then_stm, then_level, then_frags) =
                super::trans_stm(then_, test_level, value_env, breaks_stack, test_frags)?;
            let (then_label, join_label) = (newlabel(), newlabel());
            let compare_exp = BINOP(GE, Box::new(test_exp), Box::new(CONST(1)));
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        compare_exp,
                        then_label.clone(),
                        join_label.clone(),
                    ),
                    LABEL(then_label.clone()),
                    then_stm,
                    LABEL(join_label.clone()),
                ]),
                then_level,
                then_frags,
            ))
        }
        Exp::If {
            test,
            then_,
            else_: Some(else_),
        } => {
            let (test_exp, test_level, test_frags) = super::trans_exp(
                &**test,
                level,
                value_env,
                breaks_stack,
                frags,
            )?;
            let (then_stm, then_level, then_frags) = super::trans_stm(
                then_,
                test_level,
                value_env,
                breaks_stack,
                test_frags,
            )?;
            let (else_stm, else_level, else_frags) =
                super::trans_stm(else_, then_level, value_env, breaks_stack, then_frags)?;
            let (then_label, join_label, else_label) = (newlabel(), newlabel(), newlabel());
            let compare_exp = BINOP(GE, Box::new(test_exp), Box::new(CONST(1)));
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        compare_exp,
                        then_label.clone(),
                        else_label.clone(),
                    ),
                    LABEL(then_label.clone()),
                    then_stm,
                    JUMP(NAME(join_label.clone()), vec![join_label.clone()]),
                    LABEL(else_label.clone()),
                    else_stm,
                    LABEL(join_label.clone()),
                ]),
                else_level,
                else_frags,
            ))
        }
        _ => panic!("not an if"),
    }
}
