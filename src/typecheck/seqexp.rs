use crate::ast::*;
use crate::typecheck::*;

pub fn typecheck(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    match exp { Exp {node: _Exp::SeqExp(exps), ..} => {
        let mut seq_type : Tipo = TUnit;
        if exps.is_empty() {
            panic!("empty seqexp");
        }
        for exp in exps {
            seq_type = type_exp(exp, &type_env, value_env)?
        }
        Ok(seq_type)
    }
    _ => panic!("delegation panic on seqexp::tipar")
    }
}
