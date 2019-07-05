use std::collections::HashMap;

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

#[derive(Debug, PartialEq)]
pub enum R {
    RO,
    RW
}

#[derive(Debug)]
pub enum Tipo {
    TUnit,
    TNil,
    TInt(R),
    TString,
    TArray(Box<Tipo>, Box<()>),
    TRecord(Vec<Tipo>, Box<()>),
    TTipo(String)
}

impl PartialEq for Tipo {
    fn eq(&self, other: &Self) -> bool {
        use Tipo::*;
        match (self, other) {
            (TRecord(_, _), TNil) => true,
            (TNil, TRecord(_, _)) => true,
            (TRecord(_, u1), TRecord(_, u2 )) => u1 == u2,
            (TArray(_, u1), TArray(_, u2)) => u1 == u2,
            (TInt(_),TInt(_)) => true,
            (TTipo(_), _) => panic!("Estamos comparando un TTipo"),
            (_, TTipo(_)) => panic!("Estamos comparando un TTipo"),
            (a,b) => a == b,
        }
    }
}

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

pub fn not_rel(ro : RelOp) -> RelOp {
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
pub struct Level {
    // completar de tigertrans
}
pub enum Access {
    InFrame(i32),
    InReg(Label)
}

pub enum EnvEntry {
    Var {
        ty: Tipo,
        access: Access,
        level: i32,
    },
    Func {
        level: Level,
        label: Label,
        formals: Vec<Tipo>,
        result: Tipo,
        external: bool
    }
}

pub type TypeEnviroment = HashMap<Symbol, Tipo>;
pub type ValueEnviroment = HashMap<Symbol, EnvEntry>;

pub fn tipar_exp(exp : Exp, type_env : TypeEnviroment, value_env: ValueEnviroment) -> Tipo {
    use _Exp::*;
    match exp {
        Exp {node: _exp, pos: _} => match _exp {
            VarExp(_) => varexp::tipar(exp, type_env, value_env),
            UnitExp => unitexp::tipar(exp, type_env, value_env),
            NilExp => nilexp::tipar(exp, type_env, value_env),
            IntExp(_) => intexp::tipar(exp, type_env, value_env),
            StringExp(_) => stringexp::tipar(exp, type_env, value_env),
            CallExp{func: _, args: _} => callexp::tipar(exp, type_env, value_env),
            OpExp{left: _, oper: _, right: _} => opexp::tipar(exp, type_env, value_env),
            AssignExp{var: _, exp: _} => assignexp::tipar(exp, type_env, value_env),
            RecordExp{fields: _, typ: _} => recordexp::tipar(exp, type_env, value_env),
            SeqExp(_) => seqexp::tipar(exp, type_env, value_env),
            IfExp{test: _, then_: _, else_: _} => ifexp::tipar(exp, type_env, value_env),
            WhileExp{test: _, body: _} => whileexp::tipar(exp, type_env, value_env),
            ForExp{var: _, escape: _, lo: _, hi: _, body: _} => forexp::tipar(exp, type_env, value_env),
            LetExp{decs: _, body: _} => letexp::tipar(exp, type_env, value_env),
            BreakExp => breakexp::tipar(exp, type_env, value_env),
            ArrayExp{typ: _, size: _, init: _} => arrayexp::tipar(exp, type_env, value_env),
        }
    }
}

pub fn trans_exp(exp : Exp) -> ExpInterm {
    use _Exp::*;
    match exp {
        Exp {node: _exp, pos: _} => match _exp {
            VarExp(_) => varexp::traducir(exp),
            UnitExp => unitexp::traducir(exp),
            NilExp => nilexp::traducir(exp),
            IntExp(_) =>  intexp::traducir(exp),
            StringExp(_) => stringexp::traducir(exp),
            CallExp{func: _, args: _} => callexp::traducir(exp),
            OpExp{left: _, oper: _, right: _} => opexp::traducir(exp),
            AssignExp{var: _, exp: _} => assignexp::traducir(exp),
            RecordExp{fields: _, typ: _} => recordexp::traducir(exp),
            SeqExp(_) => seqexp::traducir(exp),
            IfExp{test: _, then_: _, else_: _} => ifexp::traducir(exp),
            WhileExp{test: _, body: _} => whileexp::traducir(exp),
            ForExp{var: _, escape: _, lo: _, hi: _, body: _} => forexp::traducir(exp),
            LetExp{decs: _, body: _} => letexp::traducir(exp),
            BreakExp => breakexp::traducir(exp),
            ArrayExp{typ: _, size: _, init: _} => arrayexp::traducir(exp),
        }
    }
}