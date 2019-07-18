extern crate uid;
use std::collections::HashMap;
use std::result::Result;

use super::super::ast::tigerabs::*;
use super::super::ast::position::Pos;
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


// Detalles faltantes:
//      tipoReal

#[derive(Debug, PartialEq, Clone)]
pub enum R {
    RO,
    RW
}

pub type TypeId = uid::Id<u16>;

#[derive(Debug, Clone)]
pub enum Tipo {
    TUnit,
    TNil,
    TInt(R),
    TString,
    TArray(Box<Tipo>, TypeId), 
    TRecord(Vec<(Box<String>, Box<Tipo>, u8)>, TypeId), 
    TTipo(String)
}

#[derive(Clone)]
pub enum EnvEntry {
    Var {
        ty: Tipo,
        access: Access,
        level: i32,
    },
    Func {
        // level: Level,
        label: Label,
        formals: Vec<Tipo>,
        result: Tipo,
        external: bool
    }
}

pub type TypeEnviroment = HashMap<Symbol, Tipo>;
pub type ValueEnviroment = HashMap<Symbol, EnvEntry>;

pub fn initial_type_env() -> TypeEnviroment {
    let mut type_env = TypeEnviroment::new();
    type_env.insert(Symbol::from("int"), Tipo::TInt(R::RW));
    type_env.insert(Symbol::from("string"), Tipo::TString);
    return type_env;
}

pub fn initial_value_env() -> ValueEnviroment {
    use Tipo::*;
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        label: Label::from("print"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("flush"), Func {
        label: Label::from("flush"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("getchar"), Func {
        label: Label::from("getchar"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("ord"), Func {
        label: Label::from("ord"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("chr"), Func {
        label: Label::from("chr"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("size"), Func {
        label: Label::from("size"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("substring"), Func {
        label: Label::from("substring"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("concat"), Func {
        label: Label::from("concat"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("not"), Func {
        label: Label::from("not"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    value_env.insert(Symbol::from("exit"), Func {
        label: Label::from("exit"),
        formals: vec![TString],
        result: TUnit,
        external: true
    });
    return value_env;
}

#[derive(Debug, Clone)]
pub enum TypeError {
    UndeclaredSimpleVar(Pos),
    UndeclaredFunction(Pos),
    UndeclaredType(Pos),
    FieldDoesNotExist(Pos),
    NotSimpleVar(Pos),
    NotFunctionVar(Pos),
    NotRecordType(Pos),
    NotArrayType(Pos),
    SunscriptNotInteger(Pos),
    TooManyArguments(Pos),
    TooFewArguments(Pos),
    TypeMismatch(Pos),
    NonIntegerCondition(Pos),
    NonIntegerForRange(Pos),
    ThenElseTypeMismatch(Pos),
    ReadOnlyAssignment(Pos),
    //NonUnitBody(Pos),
    NonIntegerSize(Pos),
    InvalidCallArgument(Pos),
    MissingRecordField(Pos),
    NonIntegerOperand(Pos)
}

impl PartialEq for Tipo {
    fn eq(&self, other: &Self) -> bool {
        use Tipo::*;
        match (self, other) {
            (TUnit, TUnit) => true,
            (TString, TString) => true,
            (TRecord(_, _), TNil) => true,
            (TNil, TRecord(_, _)) => true,
            (TRecord(_, uid1), TRecord(_, uid2 )) => uid1 == uid2,
            (TArray(_, uid1), TArray(_, uid2)) => uid1 == uid2,
            (TInt(_),TInt(_)) => true,
            (TTipo(s), TTipo(t)) => s == t,
            (TTipo(_), _) => panic!("Estamos comparando un TTipo"),
            (_, TTipo(_)) => panic!("Estamos comparando un TTipo"),
            (_, _) => false,
        }
    }
}

pub fn tipo_real(t: Tipo, type_env: TypeEnviroment) -> Option<Tipo> {
    match t {
        Tipo::TTipo(alias_type_symbol) => {
            match type_env.get(&alias_type_symbol) {
                Some(real_type) => Some(real_type.clone()),
                None => None
            }
        },
        tt => Some(tt)
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

#[derive(Clone)]
pub struct Level {
    // completar de tigertrans
}

#[derive(Clone)]
pub enum Access {
    InFrame(i32),
    InReg(Label)
}

pub fn tipar_exp(exp : Exp, type_env : TypeEnviroment, value_env: ValueEnviroment) -> Result<Tipo, TypeError> {
    use _Exp::*;
    match exp {
        Exp {node: _exp, pos: _pos} => match _exp {
            VarExp(_) => varexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            UnitExp => unitexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            NilExp => nilexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            IntExp(_) => intexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            StringExp(_) => stringexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            CallExp{func: _, args: _} => callexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            OpExp{left: _, oper: _, right: _} => opexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            AssignExp{var: _, exp: _} => assignexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            RecordExp{fields: _, typ: _} => recordexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            SeqExp(_) => seqexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            IfExp{test: _, then_: _, else_: _} => ifexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            WhileExp{test: _, body: _} => whileexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            ForExp{var: _, escape: _, lo: _, hi: _, body: _} => forexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            LetExp{decs: _, body: _} => letexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            BreakExp => breakexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
            ArrayExp{typ: _, size: _, init: _} => arrayexp::tipar(Exp{node: _exp, pos:_pos}, type_env, value_env),
        }
    }
}

pub fn trans_exp(exp : Exp) -> ExpInterm {
    use _Exp::*;
    match exp {
        Exp {node: _exp, pos: _pos} => match _exp {
            VarExp(_) => varexp::traducir(Exp{node: _exp, pos:_pos}),
            UnitExp => unitexp::traducir(Exp{node: _exp, pos:_pos}),
            NilExp => nilexp::traducir(Exp{node: _exp, pos:_pos}),
            IntExp(_) =>  intexp::traducir(Exp{node: _exp, pos:_pos}),
            StringExp(_) => stringexp::traducir(Exp{node: _exp, pos:_pos}),
            CallExp{func: _, args: _} => callexp::traducir(Exp{node: _exp, pos:_pos}),
            OpExp{left: _, oper: _, right: _} => opexp::traducir(Exp{node: _exp, pos:_pos}),
            AssignExp{var: _, exp: _} => assignexp::traducir(Exp{node: _exp, pos:_pos}),
            RecordExp{fields: _, typ: _} => recordexp::traducir(Exp{node: _exp, pos:_pos}),
            SeqExp(_) => seqexp::traducir(Exp{node: _exp, pos:_pos}),
            IfExp{test: _, then_: _, else_: _} => ifexp::traducir(Exp{node: _exp, pos:_pos}),
            WhileExp{test: _, body: _} => whileexp::traducir(Exp{node: _exp, pos:_pos}),
            ForExp{var: _, escape: _, lo: _, hi: _, body: _} => forexp::traducir(Exp{node: _exp, pos:_pos}),
            LetExp{decs: _, body: _} => letexp::traducir(Exp{node: _exp, pos:_pos}),
            BreakExp => breakexp::traducir(Exp{node: _exp, pos:_pos}),
            ArrayExp{typ: _, size: _, init: _} => arrayexp::traducir(Exp{node: _exp, pos:_pos}),
        }
    }
}