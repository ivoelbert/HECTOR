use std::fmt::{self, Debug, Formatter};

use super::position::WithPos;

pub type Symbol = String;

#[derive(Debug)]
pub enum Var<'a> {
    SimpleVar(Symbol),
    FieldVar(Box<Var<'a>>, Symbol),
    SubscriptVar(Box<Var<'a>>, Box<Exp<'a>>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum _Exp<'a> {
    VarExp(Var<'a>),
    UnitExp,
    NilExp,
    IntExp(i32),
    StringExp(String),
    CallExp {
        func: Symbol,
        args: Vec<Box<Exp<'a>>>,
    },
    OpExp {
        left: Box<Exp<'a>>,
        oper: Oper,
        right: Box<Exp<'a>>,
    },
    RecordExp {
        fields: Vec<(Symbol, Box<Exp<'a>>)>,
        typ: Symbol,
    },
    SeqExp(Vec<Box<Exp<'a>>>),
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
        escape: &'a mut bool,
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

pub type Exp<'a> = WithPos<_Exp<'a>>;
impl<'a> Debug for Exp<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

#[derive(Debug)]
pub struct _FunctionDec<'a> {
    name: Symbol,
    params: Vec<Field<'a>>,
    result: Option<Symbol>,
    body: Box<Exp<'a>>,
}

#[derive(Debug)]
pub struct _VarDec<'a> {
    name: Symbol,
    escape: &'a mut bool,
    typ: Option<Symbol>,
    init: Box<Exp<'a>>,
}

#[derive(Debug)]
pub struct _TypeDec<'a> {
    name: Symbol,
    ty: Ty<'a>,
}

#[derive(Debug)]
pub enum Dec<'a> {
    FunctionDec(Vec<_FunctionDec<'a>>),
    VarDec(_VarDec<'a>),
    TypeDec(Vec<_TypeDec<'a>>),
}

#[derive(Debug)]
pub enum Ty<'a> {
    NameTy(Symbol),
    RecordTy(Vec<Box<Field<'a>>>),
    ArrayTy(Symbol),
}

#[derive(Debug)]
pub struct Field<'a> {
    name: Symbol,
    escape: &'a mut bool,
    typ: Ty<'a>,
}

#[derive(Debug)]
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