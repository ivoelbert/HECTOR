use crate::ast::*;
use crate::tree::*;

pub fn trans_exp(
    AST { node, .. }: &AST,
    level: Level,
    value_env: &ValueEnviroment,
    breaks_stack: &[Option<Label>],
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
            let (then_label, join_label, else_label) = (unique_named_label("-then"), unique_named_label("-join"), unique_named_label("-else"));
            let result = unique_named_local("-ifresult");
            Ok((
                ESEQ(
                    Box::new(Tree::seq(vec![
                        CJUMP(
                            GE, Box::new(test_exp), Box::new(CONST(1)),
                            then_label.clone(),
                            else_label.clone(),
                        ),
                        LABEL(then_label),
                        Move!(LOCAL(result.clone()), then_exp),
                        JUMP(NAME(join_label.clone()), vec![join_label.clone()]),
                        LABEL(else_label),
                        Move!(LOCAL(result.clone()), else_exp),
                        LABEL(join_label),
                    ])),
                    Box::new(LOCAL(result)),
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
    breaks_stack: &[Option<Label>],
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
            let (then_label, join_label) = (unique_named_label("-then"), unique_named_label("-else"));
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        GE, Box::new(test_exp), Box::new(CONST(1)),
                        then_label.clone(),
                        join_label.clone(),
                    ),
                    LABEL(then_label),
                    then_stm,
                    LABEL(join_label),
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
            let (then_label, join_label, else_label) = (unique_named_label("-then"), unique_named_label("-join"), unique_named_label("-else"));
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        GE, Box::new(test_exp), Box::new(CONST(1)),
                        then_label.clone(),
                        else_label.clone(),
                    ),
                    LABEL(then_label),
                    then_stm,
                    JUMP(NAME(join_label.clone()), vec![join_label.clone()]),
                    LABEL(else_label),
                    else_stm,
                    LABEL(join_label),
                ]),
                else_level,
                else_frags,
            ))
        }
        _ => panic!("not an if"),
    }
}
