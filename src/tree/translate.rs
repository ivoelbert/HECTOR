#![allow(clippy::pub_enum_variant_names)]
extern crate uid;
use super::super::ast::tigerabs::*;
use super::intexp;
use super::opexp;
use super::recordexp;
use super::seqexp;
use super::assignexp;
use super::ifexp;
use super::whileexp;
use super::forexp;
use super::letexp;
use super::arrayexp;
use super::varexp;
use super::nilexp;
use super::unitexp;
use super::stringexp;
use super::callexp;
use super::breakexp;

#[derive(Debug)]
pub enum ExpInterm {
    CONST(i32),
    NAME(Label),
    TEMP(Label),
    BINOP(BinOp, Box<ExpInterm>, Box<ExpInterm>),
    MEM(Box<ExpInterm>),
    CALL(Box<ExpInterm>, Vec<ExpInterm>),
    ESEQ(Box<Stm>, Box<ExpInterm>)
}

#[derive(Debug)]
pub enum Stm {
    EXP(Box<ExpInterm>),
    JUMP(ExpInterm, Vec<Label>),
    CJUMP(RelOp, ExpInterm, ExpInterm, Label, Label),
    SEQ(Box<Stm>, Box<Stm>),
    LABEL(Label)
}

#[derive(Debug)]
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
    XOR
}

#[derive(Debug)]
pub enum RelOp {
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

pub fn not_rel(ro : &RelOp) -> RelOp {
    use RelOp::*;
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
    }
}

pub type Label = String;
pub type Temp = String;

#[derive(Clone)]
pub struct Level {
    // completar de tigertrans
}

#[derive(Clone, Debug)]
pub enum Access {
    InFrame(i32),
    InReg(Label)
}

pub fn trans_exp(exp : Exp) -> ExpInterm {
    use _Exp::*;
    match exp {
        Exp {node: exp, pos} => match exp {
            VarExp(_) => varexp::translate(Exp{node: exp, pos}),
            UnitExp => unitexp::translate(Exp{node: exp, pos}),
            NilExp => nilexp::translate(Exp{node: exp, pos}),
            IntExp(_) =>  intexp::translate(Exp{node: exp, pos}),
            StringExp(_) => stringexp::translate(Exp{node: exp, pos}),
            CallExp{..} => callexp::translate(Exp{node: exp, pos}),
            OpExp{..} => opexp::translate(Exp{node: exp, pos}),
            AssignExp{..} => assignexp::translate(Exp{node: exp, pos}),
            RecordExp{..} => recordexp::translate(Exp{node: exp, pos}),
            SeqExp(_) => seqexp::translate(Exp{node: exp, pos}),
            IfExp{..} => ifexp::translate(Exp{node: exp, pos}),
            WhileExp{..} => whileexp::translate(Exp{node: exp, pos}),
            ForExp{..} => forexp::translate(Exp{node: exp, pos}),
            LetExp{..} => letexp::translate(Exp{node: exp, pos}),
            BreakExp => breakexp::translate(Exp{node: exp, pos}),
            ArrayExp{..} => arrayexp::translate(Exp{node: exp, pos}),
        }
    }
}