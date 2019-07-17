use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn tipar(exp: Exp, type_env: TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    match exp { Exp {node: _Exp::WhileExp {test, body}, pos} => 
        match tipar_exp(*body, type_env.clone(), value_env.clone()) {
            // El ML dice que el body tiene que ser TUnit pero me parece una pavada eso.
            Ok(_) => match tipar_exp(*test, type_env.clone(), value_env.clone()) {
                Ok(Tipo::TInt(_)) => Ok(Tipo::TUnit),
                // Aca habria que tener en cuenta el tipoReal
                Ok(_) => Err(TypeError::NonIntegerCondition(pos)),
                Err(type_error) => Err(type_error)
            },
            Err(type_error) => return Err(type_error)
        }
        _ => panic!("le llego cualquier cosa a whileexp::tipar")
    }
}

pub fn traducir(exp: Exp) -> ExpInterm {
    return ExpInterm::CONST(0);
}