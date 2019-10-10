use crate::ast::*;
use crate::tree::translate::*;

pub fn trans_exp(
    Exp { node, .. }: &Exp,
    value_env: &ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    prev_frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Array { size, init, .. } => {
            let (init_exp, init_frags) = super::trans_exp(init, value_env, breaks_stack.clone(), prev_frags)?;
            let (size_exp, size_frags) = super::trans_exp(size, value_env, breaks_stack, init_frags)?;
            Ok((
                Frame::external_call(String::from("allocArray"), vec![size_exp, init_exp]),
                size_frags,
            ))
        }
        _ => panic!(),
    }
}
