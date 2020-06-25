#![allow(clippy::pub_enum_variant_names)]
use std::collections::HashMap;
use serde::{Serialize, Serializer};
extern crate snowflake;
pub use std::sync::Arc;

use crate::ast::*;
use crate::externals::{External, ArgumentType, EXTERNALS};

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
/// Write permissions for an int value
pub enum R {
    /// Read-only
    RO,
    /// Read-write
    RW
}

/// Unique identifier for Records and Arrays
pub type TypeId = snowflake::ProcessUniqueId;

/// Generate new type id for a Record or Array
pub fn newtypeid() -> TypeId {
    snowflake::ProcessUniqueId::new()
}

/// Types in the Tiger language
#[derive(Debug, Clone)]
pub enum TigerType {
    /// as in `()`
    TUnit,
    /// as in `nil`
    TNil,
    /// as in `3`
    TInt(R),
    /// as in `"perro`
    TString,
    /// as in `arrtype1 [10] of 0`
    TArray(Arc<TigerType>, TypeId),
    /// as in `{name : string, address : string, id : int, age : int}`
    TRecord(Vec<(Symbol, RecordFieldType, i32)>, TypeId),
    /// Type synonym
    Internal(String),
    /// This struct still has not been typed yet. The parser gives this type to all nodes in the AST
    Untyped,
}

#[derive(Debug, Clone)]
pub enum RecordFieldType {
    Record(TypeId),
    Type(Arc::<TigerType>)
}

impl PartialEq for RecordFieldType {
    fn eq(&self, other: &Self) -> bool {
        use RecordFieldType::*;
        match (self, other) {
            (Record(id1), Record(id2)) => id1 == id2,
            (Record(..), Type(t2)) => if let TigerType::TNil = **t2 { true } else { false },
            (Type(t1), Record(..)) => if let TigerType::TNil = **t1 { true } else { false },
            (Type(t1), Type(t2)) => t1 == t2,
        }
    }
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

/// Converts an internal type to the logical type
pub fn tipo_real(t: Arc<TigerType>, tenv: &TypeEnviroment) -> Arc<TigerType> {
    match &*t {
        TigerType::Internal(s) => match tenv.get(s) {
            Some(tipo) => Arc::clone(&tipo),
            None => panic!("Undefined")
        },
        _ => t
    }
}

/// Returns true iif the type is an Int
pub fn es_int(t: &TigerType) -> bool {
    if let TigerType::TInt(_) = *t {
        true
    } else { false }
}

/// An entry in our `TypeEnviroment` table.
#[derive(Clone, Debug)]
pub enum EnvEntry {
    /// A declared varaible
    Var {
        /// The type of the variable
        ty: Arc<TigerType>,
    },
    /// A declared function
    Func {
        /// The types of the arguments of the function
        formals: Vec<Arc<TigerType>>,
        /// The type of the return value of the function
        result: Arc<TigerType>,
    }
}

/// A table where we store the types that are declared as this point in typechecking.
///
/// When a type is used in a declaration, we look in this table and raise a `TypeError` if it's not found.
type TypeEnviroment = HashMap<Symbol, Arc<TigerType>>;
/// A table where we store the values that are declared as this point in typechecking.
///
/// When a variable or function is used somewhere in the code, we check this table and raise `TypeError` if it's not found.
type ValueEnviroment = HashMap<Symbol, EnvEntry>;

/// Generate a `TypeEnv` that contains integers and strings
fn initial_type_env() -> TypeEnviroment {
    vec![
        (Symbol::from("int"), Arc::new(TigerType::TInt(R::RW))),
        (Symbol::from("string"), Arc::new(TigerType::TString))
    ]
    .into_iter()
    .collect()
}

impl From<ArgumentType> for TigerType {
    fn from(arg: ArgumentType) -> Self {
        match arg {
            ArgumentType::String => TigerType::TString,
            ArgumentType::Int => TigerType::TInt(R::RO)
        }
    }
}

fn initial_value_env() -> ValueEnviroment {
    EXTERNALS
        .iter()
        .filter(|External {is_runtime, ..}| !is_runtime)
        .map(|External {name, arguments, return_value, ..}|
            ((*name).to_string(), EnvEntry::Func {
                formals: arguments
                    .iter()
                    .map(|arg| Arc::new(TigerType::from(*arg)))
                    .collect(),
                result: if let Some(rt) = return_value {
                    Arc::new(TigerType::from(*rt))
                } else {
                    Arc::new(TigerType::TUnit)
                }
            }))
        .collect()
}

/// Errors that the typechecker can fail with.
#[derive(Debug, Clone, Serialize)]
pub enum TypeError {
    /// Using variable that was not declared.
    UndeclaredSimpleVar(Pos),
    /// Using function that was not declared.
    UndeclaredFunction(Pos),
    /// Using type that was not declared.
    UndeclaredType(Pos),
    /// Using a field from a record that was not declared
    UndeclaredField(Pos),
    /// Tried to use an array or record as a simple variable
    NotSimpleVar(Pos),
    /// Tried to do a function call on a variable
    NotFunctionVar(Pos),
    /// Tried to access a record field on something other than a record
    NotRecordType(Pos),
    /// Tried to index something other than an array
    NotArrayType(Pos),
    /// Called a function with too many arguments
    TooManyArguments(Pos),
    /// Called a function with too few arguments
    TooFewArguments(Pos),
    /// Expected a different type
    TypeMismatch(Pos),
    /// An if-then-else with different types for each branch
    ThenElseTypeMismatch(Pos),
    /// Assigning to an Int with `R::RO`
    ReadOnlyAssignment(Pos),
    /// The bodies of for, while or if whithout else statements should type to Unit
    NonUnitBody(Pos),
    /// Type mismatch in function call argument
    InvalidCallArgument(Pos),
    /// A definition is not defining values for all record fields.
    MissingRecordField(Pos),
    /// The sizes of array definitions should type to Int
    NonIntegerSize(Pos),
    /// All conditionals should type to Int
    NonIntegerCondition(Pos),
    /// The range boundaries of for expressions should type to Int
    NonIntegerForRange(Pos),
    /// Integer operation over non integer operands
    NonIntegerOperand(Pos),
    /// The subscript of a field varaible should type to Int
    NonIntegerSubscript(Pos),
    /// Type declarations form an illicit cycle
    TypeCycle(Pos),
    /// Something is declared twice in the same block
    DuplicatedDeclarations(Pos),
    /// You can only assign nil to variables with explicit type
    UnconstrainedNilInitialization(Pos),
    /// All tiger programs should return something of Int type.
    NonIntegerProgram(Pos)
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

/// Rebuild an `AST` with the correct types given the context in the enviroments or return a `TypeError`
fn type_exp(ast : AST, type_env : &TypeEnviroment, value_env: &ValueEnviroment) -> Result<AST, TypeError> {
    let AST {node, ..} = &ast;
    match node {
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

/// Typecheck the program
pub fn typecheck(ast : AST) -> Result<AST, TypeError> {
    let typed_ast = type_exp(ast, &initial_type_env(), &initial_value_env())?;
    if *typed_ast.typ == TigerType::TInt(R::RW) {
        Ok(typed_ast)
    } else {
        Err(TypeError::NonIntegerProgram(typed_ast.pos))
    }
}