use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    use Tipo::*;
    match exp { Exp {node: _Exp::SeqExp(exps), pos: _} => {
        let mut seq_type : Tipo = TUnit;
        if exps.len() == 0 {
            panic!("0 length SeqExp");
        }
        for exp in exps {
            match tipar_exp(*exp, type_env.clone(), value_env.clone()) {
                Ok(t) => seq_type = t,
                Err(type_error) => return Err(type_error)
            }
        }
        Ok(seq_type)
    }
    _ => panic!("delegation panic on seqexp::tipar")
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}