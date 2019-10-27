use crate::ast::*;
use crate::tree::*;

pub fn trans_exp<'a>(
    Exp { node, .. }: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::If {
            test,
            then_,
            else_: Some(else_),
        } => {
            let (test_exp, test_frags) = super::trans_exp(
                &**test,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                frags,
            )?;
            let (then_exp, then_frags) = super::trans_exp(
                then_,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                test_frags,
            )?;
            let (else_exp, else_frags) =
                super::trans_exp(else_, levels, value_env, breaks_stack, then_frags)?;
            let (then_label, join_label, else_label) = (newlabel(), newlabel(), newlabel());
            let result = newtemp();
            Ok((
                ESEQ(
                    Box::new(Tree::seq(vec![
                        CJUMP(
                            GE,
                            test_exp,
                            CONST(1),
                            then_label,
                            else_label,
                        ),
                        LABEL(then_label),
                        Move!(TEMP(result), then_exp),
                        JUMP(NAME(join_label), vec![join_label]),
                        LABEL(else_label),
                        Move!(TEMP(result), else_exp),
                        LABEL(join_label),
                    ])),
                    Box::new(TEMP(result)),
                ),
                else_frags,
            ))
        }
        _ => panic!("not an if"),
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
        _Exp::If {
            test,
            then_,
            else_: None,
        } => {
            let (test_exp, test_frags) = super::trans_exp(
                &**test,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                frags,
            )?;
            let (then_stm, then_frags) =
                super::trans_stm(then_, levels, value_env, breaks_stack, test_frags)?;
            let (then_label, join_label) = (newlabel(), newlabel());
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        GE,
                        test_exp,
                        CONST(1),
                        then_label,
                        join_label,
                    ),
                    LABEL(then_label),
                    then_stm,
                    LABEL(join_label),
                ]),
                then_frags,
            ))
        }
        _Exp::If {
            test,
            then_,
            else_: Some(else_),
        } => {
            let (test_exp, test_frags) = super::trans_exp(
                &**test,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                frags,
            )?;
            let (then_stm, then_frags) = super::trans_stm(
                then_,
                levels.clone(),
                value_env.clone(),
                breaks_stack.clone(),
                test_frags,
            )?;
            let (else_stm, else_frags) =
                super::trans_stm(else_, levels, value_env, breaks_stack, then_frags)?;
            let (then_label, join_label, else_label) = (newlabel(), newlabel(), newlabel());
            Ok((
                Tree::seq(vec![
                    CJUMP(
                        GE,
                        test_exp,
                        CONST(1),
                        then_label,
                        else_label,
                    ),
                    LABEL(then_label),
                    then_stm,
                    JUMP(NAME(join_label), vec![join_label]),
                    LABEL(else_label),
                    else_stm,
                    LABEL(join_label),
                ]),
                else_frags,
            ))
        }
        _ => panic!("not an if"),
    }
}
