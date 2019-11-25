use super::*;

pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment)
-> Result<AST, TypeError> {
    match node {
        Exp::Seq(exps) => {
            assert!(!exps.is_empty());
            let typed_seq = exps
                .into_iter()
                .map(|exp| type_exp(exp, type_env, value_env))
                .collect::<Result<Vec<AST>, TypeError>>()?;
            let typ = typed_seq.last().unwrap().typ.clone();
            Ok(AST {
                node: Exp::Seq(typed_seq),
                pos,
                typ
            })
        }
        _ => panic!("delegation panic on seqexp::tipar")
    }
}
