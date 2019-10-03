#![allow(clippy::pub_enum_variant_names)]
extern crate uid;
use std::collections::HashMap;

use crate::ast::*;
use crate::tree::{Access, Label};

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

#[derive(Debug, PartialEq, Clone)]
pub enum R {
    RO,
    RW
}

pub type TypeId = uid::Id<u16>;

#[derive(Debug, Clone)]
pub enum TigerType {
    TUnit,
    TNil,
    TInt(R),
    TString,
    TArray(Box<TigerType>, TypeId),
    TRecord(Vec<(Box<String>, Box<TigerType>, u8)>, TypeId),
    TipoInterno(String)
}

pub fn tipo_real(t: TigerType, tenv: &TypeEnviroment) -> TigerType {
    match &t {
        TigerType::TipoInterno(s) => match tenv.get(s) {
            Some(tipo) => tipo.clone(),
            None => panic!("at the tipo")
        },
        _ => t
    }
}

pub fn es_int(t: &TigerType) -> bool {
    match *t {
        TigerType::TInt(_) => true,
        _ => false
    }
}

#[derive(Clone, Debug)]
pub enum EnvEntry {
    Var {
        ty: TigerType,
        access: Access,
        level: i32,
    },
    Func {
        // level: Level,
        label: Label,
        formals: Vec<TigerType>,
        result: TigerType,
        external: bool
    }
}

pub type TypeEnviroment = HashMap<Symbol, TigerType>;
pub type ValueEnviroment = HashMap<Symbol, EnvEntry>;

pub fn initial_type_env() -> TypeEnviroment {
    let mut type_env = TypeEnviroment::new();
    type_env.insert(Symbol::from("int"), TigerType::TInt(R::RW));
    type_env.insert(Symbol::from("string"), TigerType::TString);
    type_env
}

// revisar valores de retorno de estas
pub fn initial_value_env() -> ValueEnviroment {
    use TigerType::*;
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
    TypeCycle(Pos),
}

impl PartialEq for TigerType {
    fn eq(&self, other: &Self) -> bool {
        use TigerType::*;
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

pub fn type_exp(exp : &Exp, type_env : &TypeEnviroment, value_env: &ValueEnviroment) -> Result<TigerType, TypeError> {
    match exp {
        Exp {node, ..} => match node {
            _Exp::Var(..) => varexp::typecheck(exp, type_env, value_env),
            _Exp::Unit => unitexp::typecheck(exp, type_env, value_env),
            _Exp::Nil => nilexp::typecheck(exp, type_env, value_env),
            _Exp::Int(..) => intexp::typecheck(exp, type_env,&value_env),
            _Exp::String(..) => stringexp::typecheck(exp, type_env, value_env),
            _Exp::Call{..} => callexp::typecheck(exp, type_env, value_env),
            _Exp::Op{..} => opexp::typecheck(exp,&type_env, value_env),
            _Exp::Assign{..} => assignexp::typecheck(exp, type_env, value_env),
            _Exp::Record{..} => recordexp::typecheck(exp, type_env, value_env),
            _Exp::Seq(..) => seqexp::typecheck(exp, type_env, value_env),
            _Exp::If{..} => ifexp::typecheck(exp, type_env, value_env),
            _Exp::While{..} => whileexp::typecheck(exp, type_env, value_env),
            _Exp::For{..} => forexp::typecheck(exp, type_env, value_env),
            _Exp::Let{..} => letexp::typecheck(exp, type_env, value_env),
            _Exp::Break => breakexp::typecheck(exp, type_env, value_env),
            _Exp::Array{..} => arrayexp::typecheck(exp, type_env, value_env),
        }
    }
}
