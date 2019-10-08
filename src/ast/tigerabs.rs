use std::fmt::{self, Debug, Formatter};

use super::position::{Pos, WithPos};

pub type Symbol = String;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Var {
    SimpleVar(Symbol),
    FieldVar(Box<Var>, Symbol),
    SubscriptVar(Box<Var>, Box<Exp>),
}

impl Debug for Var {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            //_Exp::VarExp(var) => write!(formatter, "Var({:?})", var),
            Var::SimpleVar(sym) => write!(formatter, "{}", sym),
            Var::SubscriptVar(var, exp) => write!(formatter, "({:?}[{:?}])", *var, *exp),
            Var::FieldVar(var, sym) => write!(formatter, "({:?}.{})", *var, sym),
        }
    }
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
            _Exp::CallExp {func, args} => write!(formatter, "({}({:?}))", func, args),
            _Exp::OpExp {left, oper, right} => write!(formatter, "({:?} {:?} {:?})", left, oper, right),
            _Exp::RecordExp {fields, typ} => write!(formatter, "(Record({:?}) {{ {:?} }})", typ, fields),
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
    name: Symbol,
    params: Vec<Field>,
    result: Option<Symbol>,
    body: Box<Exp>,
}

#[derive(Debug, Clone)]
pub struct _VarDec {
    name: Symbol,
    pub escape: bool,
    typ: Option<Symbol>,
    init: Box<Exp>,
}

#[derive(Debug, Clone)]
pub struct _TypeDec {
    name: Symbol,
    ty: Ty,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Dec {
    FunctionDec(Vec<_FunctionDec>),
    VarDec(_VarDec),
    TypeDec(Vec<_TypeDec>),
}

pub fn append_dec(new_dec: Dec, decs: Vec<Dec>) -> Vec<Dec> {
    let cloned_decs = decs.clone();
    let first_dec = cloned_decs[0].clone();
    let cloned_new_dec = new_dec.clone();

    match (new_dec, first_dec) {
        (Dec::FunctionDec(new_fd), Dec::FunctionDec(fds)) => {
            // return the same decs, with the new_fd pushed into fds
            let mut new_decs: Vec<Dec> = vec![];
            for dec in cloned_decs {
                new_decs.push(dec);
            }

            let mut new_fds = vec![new_fd[0].clone()];
            for fd in fds {
                new_fds.push(fd);
            }

            new_decs[0] = Dec::FunctionDec(new_fds);

            new_decs
        },
        (Dec::TypeDec(new_td), Dec::TypeDec(tds)) => {
            // return the same decs, with the new_td pushed into tds
            let mut new_decs: Vec<Dec> = vec![];
            for dec in cloned_decs {
                new_decs.push(dec);
            }

            let mut new_tds = vec![new_td[0].clone()];
            for td in tds {
                new_tds.push(td);
            }

            new_decs[0] = Dec::TypeDec(new_tds);

            new_decs
        },
        (_, _) => {
            // return [new_dec, ...decs]
            let mut new_decs: Vec<Dec> = vec![cloned_new_dec];

            for dec in cloned_decs {
                new_decs.push(dec);
            }

            new_decs
        }
    }
}

impl _FunctionDec {
    pub fn new(name: Symbol, params: Vec<Field>, result: Option<Symbol>, body: Box<Exp>) -> _FunctionDec {
        _FunctionDec {
            name,
            params,
            result,
            body,
        }
    }
}

impl _VarDec {
    pub fn new(name: Symbol, typ: Option<Symbol>, init: Box<Exp>) -> _VarDec {
        _VarDec {
            name,
            escape: false,
            typ,
            init,
        }
    }
}

impl _TypeDec {
    pub fn new(name: Symbol, ty: Ty) -> _TypeDec {
        _TypeDec {
            name,
            ty,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ty {
    NameTy(Symbol),
    RecordTy(Vec<Field>),
    ArrayTy(Symbol),
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

// Crazy hack brought from the yacc/bison parser
pub fn var_name(var: Var) -> Symbol {
    match var {
        Var::SimpleVar(n) => n,
        _ => panic!("Crazy hack to catch array names was intersected by crappy code!"),
    }
}