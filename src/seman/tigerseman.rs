#![allow(clippy::pub_enum_variant_names)]
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
//      tipoReal y TipoInterno


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
    TipoInterno(String)
}

pub fn tipo_real(t: Tipo, tenv: &TypeEnviroment) -> Tipo {
    match &t {
        Tipo::TipoInterno(s) => match tenv.get(s) {
            Some(tipo) => tipo.clone(),
            None => panic!("at the tipo")
        },
        _ => t
    }
}

pub fn es_int(t: &Tipo) -> bool {
    match &t {
        Tipo::TInt(_) => true,
        _ => false
    }
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
    type_env
}

// revisar valores de retorno de estas
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
    value_env
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
    NonUnitBody(Pos),
    NonIntegerSize(Pos),
    InvalidCallArgument(Pos),
    MissingRecordField(Pos),
    NonIntegerOperand(Pos),
    TypeDecSortingError(Pos),
}

impl PartialEq for Tipo {
    fn eq(&self, other: &Self) -> bool {
        use Tipo::*;
        match (self, other) {
            (TUnit, TUnit)
            | (TString, TString)
            | (TRecord(_, _), TNil)
            | (TNil, TRecord(_, _))
            | (TInt(_),TInt(_)) => true,
            (TRecord(_, uid1), TRecord(_, uid2 ))
            | (TArray(_, uid1), TArray(_, uid2)) => uid1 == uid2,
            (TipoInterno(s), TipoInterno(t)) => s == t,
            (TipoInterno(_), _) => panic!("Estamos comparando un TipoInterno"), 
            (_, TipoInterno(_)) => panic!("Estamos comparando un TipoInterno"),
            (_, _) => false,
        }
    }
}

impl Tipo {
    pub fn real(&self, type_env: &TypeEnviroment) -> Option<Self> {
        match self.clone() {
            Tipo::TipoInterno(alias_type_symbol) => {
                match type_env.get(&alias_type_symbol) {
                    Some(real_type) => Some(real_type.clone()),
                    None => None
                }
            },
            tt @ Tipo::TUnit
            | tt @ Tipo::TNil
            | tt @ Tipo::TInt(..)
            | tt @ Tipo::TString
            | tt @ Tipo::TArray(..)
            | tt @ Tipo::TRecord(..) => Some(tt)
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

#[derive(Clone)]
pub enum Access {
    InFrame(i32),
    InReg(Label)
}

pub fn tipar_exp(exp : &Exp, type_env : &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Tipo, TypeError> {
    use _Exp::*;
    match exp {
        Exp {node, ..} => match node {
            VarExp(..) => varexp::tipar(exp, type_env, value_env),
            UnitExp => unitexp::tipar(exp, type_env, value_env),
            NilExp => nilexp::tipar(exp, type_env, value_env),
            IntExp(..) => intexp::tipar(exp, type_env,&value_env),
            StringExp(..) => stringexp::tipar(exp, type_env, value_env),
            CallExp{..} => callexp::tipar(exp, type_env, value_env),
            OpExp{..} => opexp::tipar(exp,&type_env, value_env),
            AssignExp{..} => assignexp::tipar(exp, type_env, value_env),
            RecordExp{..} => recordexp::tipar(exp, type_env, value_env),
            SeqExp(..) => seqexp::tipar(exp, type_env, value_env),
            IfExp{..} => ifexp::tipar(exp, type_env, value_env),
            WhileExp{..} => whileexp::tipar(exp, type_env, value_env),
            ForExp{..} => forexp::tipar(exp, type_env, value_env),
            LetExp{..} => letexp::tipar(exp, type_env, value_env),
            BreakExp => breakexp::tipar(exp, type_env, value_env),
            ArrayExp{..} => arrayexp::tipar(exp, type_env, value_env),
        }
    }
}

pub fn trans_exp(exp : Exp) -> ExpInterm {
    use _Exp::*;
    match exp {
        Exp {node: exp, pos} => match exp {
            VarExp(_) => varexp::traducir(Exp{node: exp, pos}),
            UnitExp => unitexp::traducir(Exp{node: exp, pos}),
            NilExp => nilexp::traducir(Exp{node: exp, pos}),
            IntExp(_) =>  intexp::traducir(Exp{node: exp, pos}),
            StringExp(_) => stringexp::traducir(Exp{node: exp, pos}),
            CallExp{..} => callexp::traducir(Exp{node: exp, pos}),
            OpExp{..} => opexp::traducir(Exp{node: exp, pos}),
            AssignExp{..} => assignexp::traducir(Exp{node: exp, pos}),
            RecordExp{..} => recordexp::traducir(Exp{node: exp, pos}),
            SeqExp(_) => seqexp::traducir(Exp{node: exp, pos}),
            IfExp{..} => ifexp::traducir(Exp{node: exp, pos}),
            WhileExp{..} => whileexp::traducir(Exp{node: exp, pos}),
            ForExp{..} => forexp::traducir(Exp{node: exp, pos}),
            LetExp{..} => letexp::traducir(Exp{node: exp, pos}),
            BreakExp => breakexp::traducir(Exp{node: exp, pos}),
            ArrayExp{..} => arrayexp::traducir(Exp{node: exp, pos}),
        }
    }
}