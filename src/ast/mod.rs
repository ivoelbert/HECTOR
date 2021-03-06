//! Tiger AST and parser

pub mod position;
pub mod parser;
pub mod lexer;
mod token;

pub use position::{Pos};

use std::fmt::{self, Debug, Formatter};
use serde::Serialize;
use crate::typecheck::{TigerType, Arc};

/// A symbol that appears at least once in the source code
pub type Symbol = String;

#[derive(Clone, Serialize)]
/// The Tiger Abstract Syntax Tree representing the source code
pub struct AST {
    /// The Tiger expression
    pub node: Exp,
    /// The position in the source code where this node starts
    pub pos: Pos,
    /// The type for this node, if allready typechecked, or untyped
    pub typ: Arc<TigerType>,
}

#[derive(Clone, Debug, Serialize)]
/// A Variable in the AST
pub struct Var {
    /// Kind of variable
    pub kind: VarKind,
    /// Position in source code
    pub pos: Pos,
    /// Type of the variable
    pub typ: Arc<TigerType>,
}

#[derive(Clone, Debug, Serialize)]
/// Posibles kind of variables in an AST
pub enum VarKind {
    /// As in `x`
    Simple(Symbol),
    /// As in `x.a`
    Field(Box<Var>, Symbol),
    /// As in `x[2]`
    Subscript(Box<Var>, Box<AST>),
}

#[derive(Clone, Serialize)]
pub enum Exp {
    Var(Var),
    Unit,
    Nil,
    Int(i32),
    String(String),
    Call {
        func: Symbol,
        args: Vec<AST>,
    },
    Op {
        left: Box<AST>,
        oper: Oper,
        right: Box<AST>,
    },
    Record {
        fields: Vec<(Symbol, Box<AST>)>,
        typ: Symbol,
    },
    Seq(Vec<AST>),
    Assign {
        var: Var,
        exp: Box<AST>,
    },
    If {
        test: Box<AST>,
        then_: Box<AST>,
        else_: Option<Box<AST>>,
    },
    While {
        test: Box<AST>,
        body: Box<AST>,
    },
    For {
        var: Symbol,
        escape: bool,
        lo: Box<AST>,
        hi: Box<AST>,
        body: Box<AST>,
    },
    Let {
        decs: Vec<Dec>,
        body: Box<AST>,
    },
    Break,
    Array {
        typ: Symbol,
        size: Box<AST>,
        init: Box<AST>,
    },
}

impl Debug for Exp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Exp::Var(var) => write!(formatter, "Var({:?})", var),
            Exp::Unit => write!(formatter, "UNIT"),
            Exp::Nil => write!(formatter, "NIL"),
            Exp::Int(num) => write!(formatter, "Num({:?})", num),
            Exp::String(string) => write!(formatter, "Str({:?})", string),
            Exp::Call {func, args} => write!(formatter, "{:?}({:?})", func, args),
            Exp::Op {left, oper, right} => write!(formatter, "({:?} {:?} {:?})", left, oper, right),
            Exp::Record {fields, typ, ..} => write!(formatter, "(Record({:?}) {{ {:?} }})", typ, fields),
            Exp::Seq(seq) => write!(formatter, "{:?}", seq),
            Exp::Assign {var, exp} => write!(formatter, "({:?} := {:?})", var, exp),
            Exp::If {test, then_, else_: Some(e)} => write!(formatter, "(if {:?} then {:?} else {:?})", test, then_, e),
            Exp::If {test, then_, else_: None} => write!(formatter, "(if {:?} then {:?})", test, then_),
            Exp::While {test, body} => write!(formatter, "(while({:?}) {{ {:?} }})", test, body),
            Exp::For {var, lo, hi, body, ..} => write!(formatter, "(for {:?} := {:?} to {:?} {{ {:?} }} )", var, lo, hi, body),
            Exp::Let {decs, body} => write!(formatter, "(Let {{ {:?} }} in {{ {:?} }})", decs, body),
            Exp::Break => write!(formatter, "BREAK"),
            Exp::Array {typ, size, init} => write!(formatter, "(Array({:?}) [{:?} x {:?}])", typ, size, init),
        }
    }
}

impl std::fmt::Display for Exp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Exp::Var(var) => write!(formatter, "Var({:?})", var),
            Exp::Unit => write!(formatter, "UNIT"),
            Exp::Nil => write!(formatter, "NIL"),
            Exp::Int(num) => write!(formatter, "Num({:?})", num),
            Exp::String(string) => write!(formatter, "Str({:?})", string),
            Exp::Call {func, args} => write!(formatter, "{:?}({:?})", func, args),
            Exp::Op {left, oper, right} => write!(formatter, "({:?} {:?} {:?})", left, oper, right),
            Exp::Record {fields, typ, ..} => write!(formatter, "(Record({:?}) {{ {:?} }})", typ, fields),
            Exp::Seq(seq) => write!(formatter, "{:?}", seq),
            Exp::Assign {var, exp} => write!(formatter, "({:?} := {:?})", var, exp),
            Exp::If {test, then_, else_: Some(e)} => write!(formatter, "(if {:?} then {:?} else {:?})", test, then_, e),
            Exp::If {test, then_, else_: None} => write!(formatter, "(if {:?} then {:?})", test, then_),
            Exp::While {test, body} => write!(formatter, "(while({:?}) {{ {:?} }})", test, body),
            Exp::For {var, lo, hi, body, ..} => write!(formatter, "(for {:?} := {:?} to {:?} {{ {:?} }} )", var, lo, hi, body),
            Exp::Let {decs, body} => write!(formatter, "(Let {{ {:?} }} in {{ {:?} }})", decs, body),
            Exp::Break => write!(formatter, "BREAK"),
            Exp::Array {typ, size, init} => write!(formatter, "(Array({:?}) [{:?} x {:?}])", typ, size, init),
        }
    }
}

impl Debug for AST {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

impl std::fmt::Display for AST {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct _FunctionDec {
    pub name: Symbol,
    pub params: Vec<Field>,
    pub result: Option<Symbol>,
    pub body: Box<AST>,
}

#[derive(Debug, Clone, Serialize)]
pub struct _VarDec {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Option<Symbol>,
    pub init: Box<AST>,
}

#[derive(Debug, Clone, Serialize)]
pub struct _TypeDec {
    pub name: Symbol,
    pub ty: Ty,
}

#[derive(Debug, Clone, Serialize)]
pub enum Dec {
    Function(Vec<(_FunctionDec, Pos)>),
    Var(_VarDec, Pos),
    Type(Vec<(_TypeDec, Pos)>),
}

impl _FunctionDec {
    pub fn new(name: Symbol, params: Vec<Field>, result: Option<Symbol>, body: Box<AST>) -> Self {
        Self {
            name,
            params,
            result,
            body
        }
    }
}

impl _VarDec {
    pub fn new(name: Symbol, typ: Option<Symbol>, init: Box<AST>) -> Self {
        Self {
            name,
            escape: false,
            typ,
            init,
        }
    }
}

impl _TypeDec {
    pub fn new(name: Symbol, ty: Ty) -> Self {
        Self {
            name,
            ty,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Ty {
    Name(Symbol),
    Record(Vec<Field>),
    Array(Symbol),
}


#[derive(Debug, Clone, Serialize)]
pub struct Field {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Ty,
}

#[derive(Clone, Copy, Serialize)]
pub enum Oper {
    Plus,
    Minus,
    Times,
    Divide,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Debug for Oper {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Oper::Plus => write!(formatter, "+"),
            Oper::Minus => write!(formatter, "-"),
            Oper::Times => write!(formatter, "*"),
            Oper::Divide => write!(formatter, "/"),
            Oper::Eq => write!(formatter, "="),
            Oper::Neq => write!(formatter, "<>"),
            Oper::Lt => write!(formatter, "<"),
            Oper::Le => write!(formatter, "<="),
            Oper::Gt => write!(formatter, ">"),
            Oper::Ge => write!(formatter, ">="),
        }
    }
}

pub fn posed_exp(exp: Exp, line: u32, column: u32) -> Box<AST> {
    let pos = Pos::new(line, column);
    let pos_exp = AST {
        node: exp,
        pos,
        typ: Arc::new(TigerType::Untyped)
    };
    Box::new(pos_exp)
}

#[allow( clippy::module_name_repetitions)]
pub fn make_ast(exp: Exp) -> AST {
    AST {node: exp, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)}
}

#[allow( clippy::module_name_repetitions)]
pub fn boxed_ast(exp: Exp) -> Box<AST> {
    Box::new(AST {node: exp, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)})
}

pub fn make_var(kind: VarKind) -> Var {
    Var {kind, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)}
}

pub fn append_dec(new_dec: Dec, decs: Vec<Dec>) -> Vec<Dec> {
    let first_dec = decs[0].clone();
    let cloned_new_dec = new_dec.clone();

    match (new_dec, first_dec) {
        (Dec::Function(new_fd), Dec::Function(fds)) => {
            // return the same decs, with the new_fd pushed into fds
            let mut new_decs: Vec<Dec> = vec![];
            for dec in decs {
                new_decs.push(dec);
            }

            let mut new_fds = vec![new_fd[0].clone()];
            for fd in fds {
                new_fds.push(fd);
            }

            new_decs[0] = Dec::Function(new_fds);

            new_decs
        },
        (Dec::Type(new_td), Dec::Type(tds)) => {
            // return the same decs, with the new_td pushed into tds
            let mut new_decs: Vec<Dec> = vec![];
            for dec in decs {
                new_decs.push(dec);
            }

            let mut new_tds = vec![new_td[0].clone()];
            for td in tds {
                new_tds.push(td);
            }

            new_decs[0] = Dec::Type(new_tds);

            new_decs
        },
        (_, _) => {
            // return [new_dec, ...decs]
            let mut new_decs: Vec<Dec> = vec![cloned_new_dec];

            for dec in decs {
                new_decs.push(dec);
            }

            new_decs
        }
    }
}

// Crazy hack brought from the yacc/bison parser
pub fn var_name(var: Var) -> Symbol {
    match var {
        Var { kind: VarKind::Simple(n), .. } => n,
        _ => panic!("Crazy hack to catch array names was intersected by crappy code!"),
    }
}