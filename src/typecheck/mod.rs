#![allow(clippy::pub_enum_variant_names)]
use std::collections::HashMap;
use serde::{Serialize, Serializer};
extern crate snowflake;
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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum R {
    RO,
    RW
}

pub type TypeId = snowflake::ProcessUniqueId;
pub fn newtypeid() -> TypeId {
    snowflake::ProcessUniqueId::new()
}

#[derive(Debug, Clone)]
pub enum TigerType {
    TUnit,
    TNil,
    TInt(R),
    TString,
    TArray(Arc<TigerType>, TypeId),
    TRecord(Vec<(String, Arc<TigerType>, i64)>, TypeId),
    Internal(String),
    Untyped,
}

impl Serialize for TigerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TigerType::TUnit => {
                serializer.serialize_str("Unit")
            }
            TigerType::TNil => {
                serializer.serialize_str("Nil")
            }
            TigerType::TString => {
                serializer.serialize_str("String")
            }
            TigerType::TInt(..) => {
                serializer.serialize_str("Int")
            }
            TigerType::TArray(..) => {
                serializer.serialize_str("Array")
            }
            TigerType::TRecord(..) => {
                serializer.serialize_str("Record")
            }
            TigerType::Internal(..) => {
                serializer.serialize_str("Internal")
            }
            TigerType::Untyped => {
                serializer.serialize_str("Untyped")
            }
        }
    }
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

#[derive(Debug, Clone, Serialize)]
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

pub fn type_exp(ast : AST, type_env : &TypeEnviroment, value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    match &ast {
        AST {node, ..} => match node {
            Exp::Var(..) => varexp::typecheck(ast, type_env, value_env),
            Exp::Unit => unitexp::typecheck(ast, type_env, value_env),
            Exp::Nil => nilexp::typecheck(ast, type_env, value_env),
            Exp::Int(..) => intexp::typecheck(ast, type_env,&value_env),
            Exp::String(..) => stringexp::typecheck(ast, type_env, value_env),
            Exp::Call{..} => callexp::typecheck(ast, type_env, value_env),
            Exp::Op{..} => opexp::typecheck(ast,&type_env, value_env),
            Exp::Assign{..} => assignexp::typecheck(ast, type_env, value_env),
            Exp::Record{..} => recordexp::typecheck(ast, type_env, value_env),
            Exp::Seq(..) => seqexp::typecheck(ast, type_env, value_env),
            Exp::If{..} => ifexp::typecheck(ast, type_env, value_env),
            Exp::While{..} => whileexp::typecheck(ast, type_env, value_env),
            Exp::For{..} => forexp::typecheck(ast, type_env, value_env),
            Exp::Let{..} => letexp::typecheck(ast, type_env, value_env),
            Exp::Break => breakexp::typecheck(ast, type_env, value_env),
            Exp::Array{..} => arrayexp::typecheck(ast, type_env, value_env),
        }
    }
}
