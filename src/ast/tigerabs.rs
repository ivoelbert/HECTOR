#![allow(clippy::pub_enum_variant_names)]
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use super::position::{Pos, WithPos};

pub type Symbol = String;

#[derive(Debug)]
pub enum Var<'a> {
    SimpleVar(Symbol),
    FieldVar(Box<Var<'a>>, Symbol),
    SubscriptVar(Box<Var<'a>>, Box<Exp<'a>>),
}

#[allow(dead_code)]
pub enum _Exp<'a> {
    VarExp(Var<'a>),
    UnitExp,
    NilExp,
    IntExp(i32),
    StringExp(String),
    CallExp {
        func: Symbol,
        args: Vec<Exp<'a>>,
    },
    OpExp {
        left: Box<Exp<'a>>,
        oper: Oper,
        right: Box<Exp<'a>>,
    },
    RecordExp {
        fields: Vec<(Symbol, Box<Exp<'a>>)>,
        typ: Symbol,
        phantom: PhantomData<&'a Exp<'a>>
    },
    SeqExp(Vec<Exp<'a>>),
    AssignExp {
        var: Var<'a>,
        exp: Box<Exp<'a>>,
    },
    IfExp {
        test: Box<Exp<'a>>,
        then_: Box<Exp<'a>>,
        else_: Option<Box<Exp<'a>>>,
    },
    WhileExp {
        test: Box<Exp<'a>>,
        body: Box<Exp<'a>>,
    },
    ForExp {
        var: Symbol,
        escape: bool,
        lo: Box<Exp<'a>>,
        hi: Box<Exp<'a>>,
        body: Box<Exp<'a>>,
    },
    LetExp {
        decs: Vec<Dec<'a>>,
        body: Box<Exp<'a>>,
    },
    BreakExp,
    ArrayExp {
        typ: Symbol,
        size: Box<Exp<'a>>,
        init: Box<Exp<'a>>,
    },
}

impl<'a> Debug for _Exp<'a> {
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

pub type Exp<'a> = WithPos<_Exp<'a>>;
impl<'a> Debug for Exp<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

#[derive(Debug)]
pub struct _FunctionDec<'a> {
    pub name: Symbol,
    pub params: Vec<Field<'a>>,
    pub result: Option<Symbol>,
    pub body: Box<Exp<'a>>,
}

#[derive(Debug)]
pub struct _VarDec<'a> {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Option<Symbol>,
    pub init: Box<Exp<'a>>,
}

#[derive(Debug, Clone)]
pub struct _TypeDec<'a> {
    pub name: Symbol,
    pub ty: Ty<'a>,
    phantom: PhantomData<&'a _TypeDec<'a>>
}

#[derive(Debug)]
pub enum Dec<'a> {
    FunctionDec(Vec<(_FunctionDec<'a>, Pos)>),
    VarDec(_VarDec<'a>, Pos),
    TypeDec(Vec<(_TypeDec<'a>, Pos)>),
}

impl<'a> _FunctionDec<'a> {
    pub fn new(name: Symbol, params: Vec<Field<'a>>, result: Option<Symbol>, body: Box<Exp<'a>>) -> Self {
        Self {
            name,
            params,
            result,
            body
        }
    }
}

impl<'a> _VarDec<'a> {
    pub fn new(name: Symbol, typ: Option<Symbol>, init: Box<Exp<'a>>) -> Self {
        Self {
            name,
            escape: false,
            typ,
            init,
        }
    }
}

impl<'a> _TypeDec<'a> {
    pub fn new(name: Symbol, ty: Ty<'a>) -> Self {
        Self {
            name,
            ty,
            phantom: PhantomData
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ty<'a> {
    Name(Symbol),
    Record(Vec<Field<'a>>),
    Array(Symbol),
}


#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Ty<'a>,
    phantom: PhantomData<&'a Field<'a>>
}

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
