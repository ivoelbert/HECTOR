
pub mod escape;
pub mod frame;
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

use crate::ast::*;

#[derive(Debug)]
pub enum Tree {
    CONST(i32),
    NAME(Label),
    TEMP(Label),
    BINOP(BinOp, Box<Tree>, Box<Tree>),
    MEM(Box<Tree>),
    CALL(Box<Tree>, Vec<Tree>),
    ESEQ(Box<Stm>, Box<Tree>)
}

#[derive(Debug)]
pub enum Stm {
    EXP(Box<Tree>),
    JUMP(Tree, Vec<Label>),
    CJUMP(RelOp, Tree, Tree, Label, Label),
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

pub enum TransError {
    BreakError,
    DivByZero
}


pub fn trans_exp(exp : Exp) -> Result<Tree, TransError> {
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