use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: &Exp, type_env: &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    match exp { Exp {node: _Exp::SeqExp(exps), ..} => {
        let mut seq_type : Tipo = TUnit;
        if exps.is_empty() {
            panic!("empty seqexp");
        }
        for exp in exps {
            seq_type = tipar_exp(exp, &type_env, value_env)?
        }
        Ok(seq_type)
    }
    _ => panic!("delegation panic on seqexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}