use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp { Exp {node: _Exp::OpExp{left, oper: _, right}, pos} => {
        let _ = match tipar_exp(*left, type_env.clone(), value_env.clone()) {
            Ok(Tipo::TInt(_)) => (),
            Ok(_) => return Err(TypeError::NonIntegerOperand(pos)),
            Err(type_error) => return Err(type_error)
        };
        let _ = match tipar_exp(*right, type_env.clone(), value_env.clone()) {
            Ok(Tipo::TInt(_)) => (),
            Ok(_) => return Err(TypeError::NonIntegerOperand(pos)),
            Err(type_error) => return Err(type_error)
        };
        Ok(Tipo::TInt(R::RW))
    }
    _ => panic!("delegation errror on opexp::tipar")
    }
}

pub fn traducir(_exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}