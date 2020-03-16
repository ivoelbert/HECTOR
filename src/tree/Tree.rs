use super::level::{Label, Temp};
use serde::{Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Exp {
    CONST(i64),
    NAME(Label),
    TEMP(Temp),
    BINOP(BinOp, Box<Exp>, Box<Exp>),
    MEM(Box<Exp>),
    CALL(String, Box<Exp>, Vec<Exp>),
    ESEQ(Box<Stm>, Box<Exp>)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Stm {
    EXP(Box<Exp>),
    MOVE(Box<Exp>, Box<Exp>),
    JUMP(Exp, Vec<Label>),
    CJUMP(Exp, Label, Label),
    SEQ(Box<Stm>, Box<Stm>),
    LABEL(Label)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum BinOp {
    PLUS,
    MINUS,
    MUL,
    DIV,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    ARSHIFT,
    XOR,
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
    ULT,
    ULE,
    UGT,
    UGE
}

pub fn not_rel(ro : &BinOp) -> BinOp {
    use BinOp::*;
    match ro {
        EQ => NE,
	    NE => EQ,
	    LT => GE,
	    GE => LT,
	    GT => LE,
	    LE => GT,
	    ULT => UGE,
	    UGE => ULT,
	    ULE => UGT,
        UGT => ULE,
        _ => panic!()
    }
}

pub fn seq(mut stms: Vec<Stm>) -> Stm {
    let maybe_stm = stms.pop();
    match maybe_stm {
        Some(s) => Stm::SEQ(Box::new(s), Box::new(seq(stms))),
        None => Stm::EXP(Box::new(Exp::CONST(0))),
    }
}


macro_rules! plus {
    ( $x:expr, $y:expr ) => (BINOP(PLUS, Box::new($x), Box::new($y)));
}

macro_rules! Move {
    ( $x:expr, $y:expr ) => (MOVE(Box::new($x), Box::new($y)));
}