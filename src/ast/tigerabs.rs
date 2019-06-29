use std::fmt::{self, Debug, Formatter};

use super::position::WithPos;

pub type Symbol = String;

#[derive(Debug)]
pub enum Var {
    PubSimpleVar(Symbol),
    FieldVar(Box<Var>, Symbol),
    SubscriptVar(Box<Var>, Box<Exp>),
}

#[derive(Debug)]
pub struct _CallExp {
    func: Symbol,
    args: Vec<Box<Exp>>,
}

#[derive(Debug)]
pub struct _OpExp {
    left: Box<Exp>,
    oper: Oper,
    right: Box<Exp>,
}

#[derive(Debug)]
pub struct _RecordExp {
    fields: Vec<(Symbol, Box<Exp>)>,
    typ: Symbol,
}

#[derive(Debug)]
pub struct _AssignExp {
    var: Var,
    exp: Box<Exp>,
}

#[derive(Debug)]
pub struct _IfExp {
    test: Box<Exp>,
    then_: Box<Exp>,
    else_: Option<Box<Exp>>,
}

#[derive(Debug)]
pub struct _WhileExp {
    test: Box<Exp>,
    body: Box<Exp>,
}

#[derive(Debug)]
pub struct _ForExp {
    var: Symbol,
    escape: &'static mut bool,
    lo: Box<Exp>,
    hi: Box<Exp>,
    body: Box<Exp>,
}

#[derive(Debug)]
pub struct _LetExp {
    decs: Vec<Dec>,
    body: Box<Exp>,
}

#[derive(Debug)]
pub struct _ArrayExp {
    typ: Symbol,
    size: Box<Exp>,
    init: Box<Exp>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum _Exp {
    VarExp(Var),
    UnitExp,
    NilExp,
    IntExp(i32),
    StringExp(String),
    CallExp {
        func: Symbol,
        args: Vec<Box<Exp>>,
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
    SeqExp(Vec<Box<Exp>>),
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
        escape: &'static mut bool,
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

pub type Exp = WithPos<_Exp>;
impl Debug for Exp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.node)
    }
}

#[derive(Debug)]
pub struct _FunctionDec {
    name: Symbol,
    params: Vec<Field>,
    result: Option<Symbol>,
    body: Box<Exp>,
}

#[derive(Debug)]
pub struct _VarDec {
    name: Symbol,
    escape: &'static mut bool,
    typ: Option<Symbol>,
    init: Box<Exp>,
}

#[derive(Debug)]
pub struct _TypeDec {
    name: Symbol,
    ty: Ty,
}

#[derive(Debug)]
pub enum Dec {
    FunctionDec(Vec<_FunctionDec>),
    VarDec(_VarDec),
    TypeDec(Vec<_TypeDec>),
}

#[derive(Debug)]
pub enum Ty {
    NameTy(Symbol),
    RecordTy(Vec<Box<Field>>),
    ArrayTy(Symbol),
}

#[derive(Debug)]
pub struct Field {
    name: Symbol,
    escape: &'static mut bool,
    typ: Ty,
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