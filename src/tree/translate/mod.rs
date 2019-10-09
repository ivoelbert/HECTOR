use crate::ast::*;
use crate::tree::*;

mod intexp;
mod opexp;
mod recordexp;
mod seqexp;
mod assignexp;
mod ifexp;
mod whileexp;
mod forexp;
mod letexp;
mod arrayexp;
mod varexp;
mod nilexp;
mod unitexp;
mod stringexp;
mod callexp;
mod breakexp;

pub fn seq(mut stms: Vec<Tree::Stm>) -> Tree::Stm {
    let maybe_stm = stms.pop();
        match maybe_stm {
            Some(s) => {
                SEQ(Box::new(s), Box::new(seq(stms)))
            }
            None => EXP(Box::new(CONST(0))),
        }
}

pub fn un_ex(exp: ExpInterm) -> Tree::Exp {
    match exp {
        Ex(e) => e,
        Nx(s) => ESEQ(Box::new(s), Box::new(CONST(0))),
        Cx(genstm) => {
            let r = newtemp();
            let t = newlabel();
            let f = newlabel();
            ESEQ(Box::new(seq(vec![
                MOVE(Box::new(TEMP(r.clone())), Box::new(CONST(1))),
                genstm(t.clone(), f.clone()),
                LABEL(f.clone()),
                MOVE(Box::new(TEMP(r.clone())), Box::new(CONST(0))),
                LABEL(f),
            ])), Box::new(TEMP(r)))
        }
    }
}

pub fn un_nx(exp: ExpInterm) -> Tree::Stm {
    match exp {
        Ex(e) => EXP(Box::new(e)),
        Nx(s) => s,
        Cx(cf) => {
            let t = newlabel();
            let f = newlabel();
            seq(vec![
                cf(t.clone(), f.clone()),
                LABEL(t),
                LABEL(f)
            ])
        }
    }
}

pub fn un_cx(exp: ExpInterm) -> Box<dyn Fn(Label, Label) -> Tree::Stm> {
    match exp {
        Ex(CONST(0)) => Box::new(|_, f| JUMP(NAME(f.clone()), vec![f])),
        Ex(CONST(_)) => Box::new(|t, _| JUMP(NAME(t.clone()), vec![t])),
        Ex(e) => Box::new(move |t, f| CJUMP(Tree::RelOp::NE, e.clone(), CONST(0), t, f)),
        Nx(..) => panic!("Erorr un_cx(NX(..))"),
        Cx(cf) => cf
    }
}

pub fn trans_exp(exp : Exp) -> Result<ExpInterm, TransError> {
    match exp {
        Exp {node: exp, pos} => match exp {
            _Exp::Var(_) => varexp::translate(Exp{node: exp, pos}),
            _Exp::Unit => unitexp::translate(Exp{node: exp, pos}),
            _Exp::Nil => nilexp::translate(Exp{node: exp, pos}),
            _Exp::Int(_) =>  intexp::translate(Exp{node: exp, pos}),
            _Exp::String(_) => stringexp::translate(Exp{node: exp, pos}),
            _Exp::Call{..} => callexp::translate(Exp{node: exp, pos}),
            _Exp::Op{..} => opexp::translate(Exp{node: exp, pos}),
            _Exp::Assign{..} => assignexp::translate(Exp{node: exp, pos}),
            _Exp::Record{..} => recordexp::translate(Exp{node: exp, pos}),
            _Exp::Seq(_) => seqexp::translate(Exp{node: exp, pos}),
            _Exp::If{..} => ifexp::translate(Exp{node: exp, pos}),
            _Exp::While{..} => whileexp::translate(Exp{node: exp, pos}),
            _Exp::For{..} => forexp::translate(Exp{node: exp, pos}),
            _Exp::Let{..} => letexp::translate(Exp{node: exp, pos}),
            _Exp::Break => breakexp::translate(Exp{node: exp, pos}),
            _Exp::Array{..} => arrayexp::translate(Exp{node: exp, pos}),
        }
    }
}