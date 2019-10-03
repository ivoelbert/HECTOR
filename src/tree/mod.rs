
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