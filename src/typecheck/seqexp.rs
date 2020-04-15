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
#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    #[test]
    #[wasm_bindgen_test]
    fn seqexp_ok() {
        let ast = make_ast(Exp::Seq(vec![
            make_ast(Exp::Int(1)),
            make_ast(Exp::Int(1)),
        ]));
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }
}
