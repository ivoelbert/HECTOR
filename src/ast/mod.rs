pub mod position;
pub mod parser;
mod token;

use std::fmt::{self, Debug, Formatter};
pub use crate::typecheck::{TigerType, Arc};
pub use position::{Pos, WithPos};

pub type Symbol = String;

#[derive(Clone)]
pub struct AST {
    pub node: Exp,
    pub pos: Pos,
    pub typ: Arc<TigerType>,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub kind: VarKind,
    pub pos: Pos,
    pub typ: Arc<TigerType>,
}

#[derive(Clone, Debug)]
pub enum VarKind {
    Simple(Symbol),
    Field(Box<Var>, Symbol),
    Subscript(Box<Var>, Box<AST>),
}

#[derive(Clone)]
pub enum Exp {
    Var(Var),
    Unit,
    Nil,
    Int(i64),
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

impl Debug for AST {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

#[derive(Debug, Clone)]
pub struct _FunctionDec {
    pub name: Symbol,
    pub params: Vec<Field>,
    pub result: Option<Symbol>,
    pub body: Box<AST>,
}

#[derive(Debug, Clone)]
pub struct _VarDec {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Option<Symbol>,
    pub init: Box<AST>,
}

#[derive(Debug, Clone)]
pub struct _TypeDec {
    pub name: Symbol,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Dec {
    FunctionDec(Vec<(_FunctionDec, Pos)>),
    VarDec(_VarDec, Pos),
    TypeDec(Vec<(_TypeDec, Pos)>),
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

#[derive(Debug, Clone)]
pub enum Ty {
    Name(Symbol),
    Record(Vec<Field>),
    Array(Symbol),
}


#[derive(Debug, Clone)]
pub struct Field {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Ty,
}

#[derive(Clone, Copy)]
pub enum Oper {
    PlusOp,
    MinusOp,
    TimesOp,
    DivideOp,
    EqOp,
    NeqOp,
    LtOp,
    LeOp,
    GtOp,
    GeOp,
}

impl Debug for Oper {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Oper::PlusOp => write!(formatter, "+"),
            Oper::MinusOp => write!(formatter, "-"),
            Oper::TimesOp => write!(formatter, "*"),
            Oper::DivideOp => write!(formatter, "/"),
            Oper::EqOp => write!(formatter, "="),
            Oper::NeqOp => write!(formatter, "<>"),
            Oper::LtOp => write!(formatter, "<"),
            Oper::LeOp => write!(formatter, "<="),
            Oper::GtOp => write!(formatter, ">"),
            Oper::GeOp => write!(formatter, ">="),
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

pub fn make_ast(exp: Exp) -> AST {
    AST {node: exp, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)}
}

pub fn boxed_ast(exp: Exp) -> Box<AST> {
    Box::new(AST {node: exp, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)})
}

pub fn make_var(kind: VarKind) -> Var {
    Var {kind, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)}
}

pub fn boxed_var(kind: VarKind) -> Box<Var> {
    Box::new(Var {kind, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)})
}