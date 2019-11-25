use super::level::{Label, Temp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    CONST(i64),
    NAME(Label),
    TEMP(Temp),
    BINOP(BinOp, Box<AST>, Box<AST>),
    MEM(Box<AST>),
    CALL(Box<AST>, Vec<AST>),
    ESEQ(Box<Stm>, Box<AST>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stm {
    EXP(Box<AST>),
    MOVE(Box<AST>, Box<AST>),
    JUMP(AST, Vec<Label>),
    CJUMP(RelOp, AST, AST, Label, Label),
    SEQ(Box<Stm>, Box<Stm>),
    LABEL(Label)
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn seq(mut stms: Vec<Stm>) -> Stm {
    let maybe_stm = stms.pop();
    match maybe_stm {
        Some(s) => Stm::SEQ(Box::new(s), Box::new(seq(stms))),
        None => Stm::EXP(Box::new(AST::CONST(0))),
    }
}


macro_rules! plus {
    ( $x:expr, $y:expr ) => (BINOP(PLUS, Box::new($x), Box::new($y)));
}

macro_rules! Move {
    ( $x:expr, $y:expr ) => (MOVE(Box::new($x), Box::new($y)));
}