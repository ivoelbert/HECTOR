#![allow(clippy::pub_enum_variant_names)]
pub mod position;
mod token;

use std::fmt::{self, Debug, Formatter};
pub use position::{Pos, WithPos};

pub type Symbol = String;

#[derive(Debug)]
#[derive(Clone)]
pub enum Var {
    SimpleVar(Symbol),
    FieldVar(Box<Var>, Symbol),
    SubscriptVar(Box<Var>, Box<Exp>),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum _Exp {
    VarExp(Var),
    UnitExp,
    NilExp,
    IntExp(i32),
    StringExp(String),
    CallExp {
        func: Symbol,
        args: Vec<Exp>,
    },
    OpExp {
        left: Box<Exp>,
        oper: Oper,
        right: Box<Exp>,
    },
    RecordExp {
        fields: Vec<(Symbol, Box<Exp>)>,
        typ: Symbol,
    },
    SeqExp(Vec<Exp>),
    AssignExp {
        var: Var,
        exp: Box<Exp>,
    },
    IfExp {
        test: Box<Exp>,
        then_: Box<Exp>,
        else_: Option<Box<Exp>>,
    },
    WhileExp {
        test: Box<Exp>,
        body: Box<Exp>,
    },
    ForExp {
        var: Symbol,
        escape: bool,
        lo: Box<Exp>,
        hi: Box<Exp>,
        body: Box<Exp>,
    },
    LetExp {
        decs: Vec<Dec>,
        body: Box<Exp>,
    },
    BreakExp,
    ArrayExp {
        typ: Symbol,
        size: Box<Exp>,
        init: Box<Exp>,
    },
}

impl Debug for _Exp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            _Exp::VarExp(var) => write!(formatter, "Var({:?})", var),
            _Exp::UnitExp => write!(formatter, "UNIT"),
            _Exp::NilExp => write!(formatter, "NIL"),
            _Exp::IntExp(num) => write!(formatter, "Num({:?})", num),
            _Exp::StringExp(string) => write!(formatter, "Str({:?})", string),
            _Exp::CallExp {func, args} => write!(formatter, "{:?}({:?})", func, args),
            _Exp::OpExp {left, oper, right} => write!(formatter, "({:?} {:?} {:?})", left, oper, right),
            _Exp::RecordExp {fields, typ, ..} => write!(formatter, "(Record({:?}) {{ {:?} }})", typ, fields),
            _Exp::SeqExp(seq) => write!(formatter, "{:?}", seq),
            _Exp::AssignExp {var, exp} => write!(formatter, "({:?} := {:?})", var, exp),
            _Exp::IfExp {test, then_, else_: Some(e)} => write!(formatter, "(if {:?} then {:?} else {:?})", test, then_, e),
            _Exp::IfExp {test, then_, else_: None} => write!(formatter, "(if {:?} then {:?})", test, then_),
            _Exp::WhileExp {test, body} => write!(formatter, "(while({:?}) {{ {:?} }})", test, body),
            _Exp::ForExp {var, lo, hi, body, ..} => write!(formatter, "(for {:?} := {:?} to {:?} {{ {:?} }} )", var, lo, hi, body),
            _Exp::LetExp {decs, body} => write!(formatter, "(Let {{ {:?} }} in {{ {:?} }})", decs, body),
            _Exp::BreakExp => write!(formatter, "BREAK"),
            _Exp::ArrayExp {typ, size, init} => write!(formatter, "(Array({:?}) [{:?} x {:?}])", typ, size, init),
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
