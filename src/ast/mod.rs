pub mod position;
mod token;

use std::fmt::{self, Debug, Formatter};
pub use position::{Pos, WithPos};

pub type Symbol = String;

#[derive(Debug)]
#[derive(Clone)]
pub enum Var {
    Simple(Symbol),
    Field(Box<Var>, Symbol),
    Subscript(Box<Var>, Box<Exp>),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum _Exp {
    Var(Var),
    Unit,
    Nil,
    Int(i64),
    String(String),
    Call {
        func: Symbol,
        args: Vec<Exp>,
    },
    Op {
        left: Box<Exp>,
        oper: Oper,
        right: Box<Exp>,
    },
    Record {
        fields: Vec<(Symbol, Box<Exp>)>,
        typ: Symbol,
    },
    Seq(Vec<Exp>),
    Assign {
        var: Var,
        exp: Box<Exp>,
    },
    If {
        test: Box<Exp>,
        then_: Box<Exp>,
        else_: Option<Box<Exp>>,
    },
    While {
        test: Box<Exp>,
        body: Box<Exp>,
    },
    For {
        var: Symbol,
        escape: bool,
        lo: Box<Exp>,
        hi: Box<Exp>,
        body: Box<Exp>,
    },
    Let {
        decs: Vec<Dec>,
        body: Box<Exp>,
    },
    Break,
    Array {
        typ: Symbol,
        size: Box<Exp>,
        init: Box<Exp>,
    },
}

impl Debug for _Exp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            _Exp::Var(var) => write!(formatter, "Var({:?})", var),
            _Exp::Unit => write!(formatter, "UNIT"),
            _Exp::Nil => write!(formatter, "NIL"),
            _Exp::Int(num) => write!(formatter, "Num({:?})", num),
            _Exp::String(string) => write!(formatter, "Str({:?})", string),
            _Exp::Call {func, args} => write!(formatter, "{:?}({:?})", func, args),
            _Exp::Op {left, oper, right} => write!(formatter, "({:?} {:?} {:?})", left, oper, right),
            _Exp::Record {fields, typ, ..} => write!(formatter, "(Record({:?}) {{ {:?} }})", typ, fields),
            _Exp::Seq(seq) => write!(formatter, "{:?}", seq),
            _Exp::Assign {var, exp} => write!(formatter, "({:?} := {:?})", var, exp),
            _Exp::If {test, then_, else_: Some(e)} => write!(formatter, "(if {:?} then {:?} else {:?})", test, then_, e),
            _Exp::If {test, then_, else_: None} => write!(formatter, "(if {:?} then {:?})", test, then_),
            _Exp::While {test, body} => write!(formatter, "(while({:?}) {{ {:?} }})", test, body),
            _Exp::For {var, lo, hi, body, ..} => write!(formatter, "(for {:?} := {:?} to {:?} {{ {:?} }} )", var, lo, hi, body),
            _Exp::Let {decs, body} => write!(formatter, "(Let {{ {:?} }} in {{ {:?} }})", decs, body),
            _Exp::Break => write!(formatter, "BREAK"),
            _Exp::Array {typ, size, init} => write!(formatter, "(Array({:?}) [{:?} x {:?}])", typ, size, init),
        }
    }
}

pub type Exp = WithPos<_Exp>;
impl Debug for Exp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

#[derive(Debug, Clone)]
pub struct _FunctionDec {
    pub name: Symbol,
    pub params: Vec<Field>,
    pub result: Option<Symbol>,
    pub body: Box<Exp>,
}

#[derive(Debug, Clone)]
pub struct _VarDec {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Option<Symbol>,
    pub init: Box<Exp>,
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
    pub fn new(name: Symbol, params: Vec<Field>, result: Option<Symbol>, body: Box<Exp>) -> Self {
        Self {
            name,
            params,
            result,
            body
        }
    }
}

impl _VarDec {
    pub fn new(name: Symbol, typ: Option<Symbol>, init: Box<Exp>) -> Self {
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

#[derive(Clone)]
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

pub fn posed_exp(exp: _Exp, line: u32, column: u32) -> Box<Exp> {
    let pos = Pos::new(line, column);
    let pos_exp = WithPos::new(exp, pos);

    Box::new(pos_exp)
}

pub fn boxed_exp(exp: _Exp) -> Box<Exp> {
    Box::new(Exp {node: exp, pos: Pos {line: 0, column: 0}})
}