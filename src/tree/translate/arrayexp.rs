use crate::ast::*;
use crate::tree::translate::*;

pub fn trans_exp<'a>(
    Exp {node, ..}: &Exp,
    levels: Vec<Level>,
    value_env: ValueEnviroment,
    breaks_stack: Vec<Option<Label>>,
    frags: Vec<Fragment>,
) -> Result<(Tree::Exp, Vec<Fragment>), TransError> {
    match node {
        _Exp::Array { size, init, .. } => {
            let (init_exp, init_frags) = super::trans_exp(init, levels.clone(), value_env.clone(), breaks_stack.clone(), frags)?;
            let (size_exp, size_frags) = super::trans_exp(size, levels, value_env.clone(), breaks_stack, init_frags)?;
            if let EnvEntry::Func {label, ..} = value_env.get("allocArray").expect("should be in initial value env") {
                Ok((
                    Frame::external_call(*label, vec![size_exp, init_exp]),
                    size_frags,
                ))
            }
            else {
                panic!("external function not found");
            }
        }
        _ => panic!(),
    }
}
