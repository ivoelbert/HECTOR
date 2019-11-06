#![allow(clippy::pub_enum_variant_names)]
extern crate uid;
use std::collections::HashMap;
pub use std::sync::{Arc, Weak};


use crate::ast::*;

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
    TArray(Arc<TigerType>, TypeId),
    TRecord(Vec<(String, Arc<TigerType>, u8)>, TypeId),
    Internal(String)
}

pub fn tipo_real(t: Arc<TigerType>, tenv: &TypeEnviroment) -> Arc<TigerType> {
    match &*t {
        TigerType::Internal(s) => match tenv.get(s) {
            Some(tipo) => tipo.clone(),
            None => panic!("Undefined")
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
        ty: Arc<TigerType>,
    },
    Func {
        formals: Vec<Arc<TigerType>>,
        result: Arc<TigerType>,
    }
}

pub type TypeEnviroment = HashMap<Symbol, Arc<TigerType>>;
pub type ValueEnviroment = HashMap<Symbol, EnvEntry>;

pub fn initial_type_env() -> TypeEnviroment {
    vec![
        (Symbol::from("int"), Arc::new(TigerType::TInt(R::RW))),
        (Symbol::from("string"), Arc::new(TigerType::TString))
    ]
    .into_iter()
    .collect()
}

pub fn initial_value_env() -> ValueEnviroment {
    use TigerType::*;
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TUnit),
    });
    value_env.insert(Symbol::from("flush"), Func {
        formals: vec![],
        result: Arc::new(TUnit),
    });
    value_env.insert(Symbol::from("getchar"), Func {
        formals: vec![],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("ord"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TInt(R::RW)),
    });
    value_env.insert(Symbol::from("chr"), Func {
        formals: vec![Arc::new(TInt(R::RW))],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("size"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TInt(R::RW)),
    });
    value_env.insert(Symbol::from("substring"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("concat"), Func {
        formals: vec![Arc::new(TString), Arc::new(TInt(R::RW)), Arc::new(TInt(R::RW))],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("not"), Func {
        formals: vec![Arc::new(TInt(R::RW))],
        result: Arc::new(TInt(R::RW)),
    });
    value_env.insert(Symbol::from("exit"), Func {
        formals: vec![Arc::new(TInt(R::RW))],
        result: Arc::new(TUnit),
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
    SubscriptNotInteger(Pos),
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
    DuplicatedDefinitions(Pos),
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
            (Internal(s), Internal(t)) => s == t,
            (Internal(_), _) => panic!("Estamos comparando un Internal"),
            (_, Internal(_)) => panic!("Estamos comparando un Internal"),
            (_, _) => false,
        }
    }
}

pub fn type_exp(exp : &Exp, type_env : &TypeEnviroment, value_env: &ValueEnviroment) -> Result<Arc<TigerType>, TypeError> {
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
