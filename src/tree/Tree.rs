use super::level::{Label, LocalTemp, GlobalTemp};
use serde::{Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Exp {
    CONST(i32),
    NAME(Label),
    LOCAL(LocalTemp),
    GLOBAL(GlobalTemp),
    BINOP(BinOp, Box<Exp>, Box<Exp>),
    MEM(Box<Exp>),
    CALL(Box<Exp>, Vec<Exp>),
    ESEQ(Box<Stm>, Box<Exp>)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Stm {
    EXP(Box<Exp>),
    MOVE(Box<Exp>, Box<Exp>),
    JUMP(Exp, Vec<Label>),
    CJUMP(BinOp, Box<Exp>, Box<Exp>, Label, Label),
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
    // ARSHIFT,
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

// pub fn not_rel(ro : BinOp) -> BinOp {
//     use BinOp::*;
//     match ro {
//         EQ => NE,
// 	    NE => EQ,
// 	    LT => GE,
// 	    GE => LT,
// 	    GT => LE,
// 	    LE => GT,
// 	    ULT => UGE,
// 	    UGE => ULT,
// 	    ULE => UGT,
//         UGT => ULE,
//         _ => panic!()
//     }
// }

pub fn seq(mut stms: Vec<Stm>) -> Stm {
    let stm =  if stms.is_empty() {Stm::EXP(Box::new(Exp::CONST(0)))} else {stms.remove(0)};
    if stms.is_empty() {
        stm
    } else {
        Stm::SEQ(Box::new(stm), Box::new(seq(stms)))
    }
}


macro_rules! plus {
    ( $x:expr, $y:expr ) => (BINOP(PLUS, Box::new($x), Box::new($y)));
}

macro_rules! Move {
    ( $x:expr, $y:expr ) => (MOVE(Box::new($x), Box::new($y)));
}