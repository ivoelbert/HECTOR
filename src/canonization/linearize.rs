use super::*;

use Tree::Stm::*;
use Tree::Exp::*;


macro_rules! nop {
    () => {
        EXP(Box::new(CONST(0)))
    };
}

fn seq(x: Tree::Stm, y: Tree::Stm) -> Tree::Stm{
    match (x, y) {
        (EXP(boxed_exp), stm) => match *boxed_exp {
            CONST(_) => stm,
            _ => SEQ(Box::new(EXP(boxed_exp)), Box::new(stm))
        },
        (stm, EXP(boxed_exp)) => match *boxed_exp {
            CONST(_) => stm,
            _ => SEQ(Box::new(stm), Box::new(EXP(boxed_exp)))
        },
        (x, y) => SEQ(Box::new(x), Box::new(y))
    }
}

fn commute(stm: &Tree::Stm, exp: &Tree::Exp) -> bool {
    fn inmut(exp: &Tree::Exp) -> bool {
        match exp {
            CONST(..) | NAME(..) => true,
            GLOBAL(l) if *l == level::FRAME_POINTER => true, // Capaz que está al pedo
            BINOP(_, x, y) => inmut(x) && inmut(y),
            _ => false
        }
    }

    match (stm, exp) {
        (_, CONST(..)) | (_, NAME(..)) => true,
        (EXP(x), y) => match &**x {
            CONST(..) => true,
            CALL(label, _) => match &**label {
                NAME(name) if name == "+check_index_array" || name == "+check_nil" => true,
                _ => false,
            },
            x => inmut(x) || inmut(y)
        }
        _ => false,

    }
}

fn reorder(mut exps: Vec<Tree::Exp>) -> (Tree::Stm, Vec<Tree::Exp>) {
    if exps.is_empty() {
        return (nop!(), vec![])
    }
    let first = exps.remove(0);
    match first {
        CALL(..) => {
            let t = level::unique_named_global("-reorder_call");
            exps.insert(0, ESEQ(
                Box::new(MOVE(
                    Box::new(LOCAL(t.clone())),
                    Box::new(first)
                )),
                Box::new(LOCAL(t))
            ));
            reorder(exps)
        }
        a => {
            let (stms, e) = do_exp(a);
            let (stms_, mut el) = reorder(exps);
            if commute(&stms_, &e) {
                el.insert(0, e);
                (seq(stms, stms_), el)
            } else {
                let t = level::unique_named_global("-reorder");
                el.insert(0, LOCAL(t.clone()));
                (seq(stms, seq(MOVE(Box::new(LOCAL(t)), Box::new(e)), stms_)), el)
            }
        }
    }
}

fn reorder_exp(exps: Vec<Tree::Exp>, build: Box<dyn FnOnce(Vec<Tree::Exp>) -> Tree::Exp>) -> (Tree::Stm, Tree::Exp) {
    let (stms, exps_) = reorder(exps);
    (stms, build(exps_))
}

fn reorder_stm(exps: Vec<Tree::Exp>, build: Box<dyn FnOnce(Vec<Tree::Exp>) -> Tree::Stm>) -> Tree::Stm {
    let (stms, exps_) = reorder(exps);
    seq(stms, build(exps_))
}

fn do_stm(stm: Tree::Stm) -> Tree::Stm {
    match stm {
        SEQ(a, b) => seq(do_stm(*a), do_stm(*b)),
        JUMP(exp, labels) => reorder_stm(
            vec![exp],
            Box::new(|mut l| {
                let exp = l.pop().expect("jump canonization");
                JUMP(exp, labels)
            })
        ),
        CJUMP(o, a, b, t, f) => reorder_stm(
            vec![*a, *b],
            Box::new(move |mut l| {
                let b = l.pop().expect("cjump canonization");
                let a = l.pop().expect("cjump canonization");
                CJUMP(o, Box::new(a), Box::new(b), t, f)
            })
        ),
        MOVE(dest, src) => match (*dest, *src) {
            (GLOBAL(t), CALL(function_label, args)) => {
                let mut exps = args;
                exps.push(*function_label);
                reorder_stm(
                    exps,
                    Box::new(|mut l| {
                        let dest = Box::new(GLOBAL(t));
                        let src = Box::new(CALL(Box::new(l.pop().expect("move canonization")), l));
                        MOVE(dest, src)
                    })
                )
            },
            (GLOBAL(t), b) =>
                reorder_stm(
                    vec![b],
                    Box::new(|mut l| {
                        let dest = Box::new(GLOBAL(t));
                        let src = Box::new(l.pop().expect("move canonization"));
                        MOVE(dest, src)
                    })
                ),
            (LOCAL(t), CALL(function_label, args)) => {
                let mut exps = args;
                exps.push(*function_label);
                reorder_stm(
                    exps,
                    Box::new(|mut l| {
                        let dest = Box::new(LOCAL(t));
                        let src = Box::new(CALL(Box::new(l.pop().expect("move canonization")), l));
                        MOVE(dest, src)
                    })
                )
            },
            (LOCAL(t), b) =>
                reorder_stm(
                    vec![b],
                    Box::new(|mut l| {
                        let dest = Box::new(LOCAL(t));
                        let src = Box::new(l.pop().expect("move canonization"));
                        MOVE(dest, src)
                    })
                ),
            (MEM(m), b) =>
                reorder_stm(
                    vec![b, *m],
                    Box::new(|mut l| {
                        let dest = Box::new(MEM(Box::new(l.pop().expect("move canonization"))));
                        let src = Box::new(l.pop().expect("move canonization"));
                        MOVE(dest, src)
                    })
            ),
            (ESEQ(s, e), b) => do_stm(SEQ(s, Box::new(MOVE(e, Box::new(b))))),
            (a, b) => reorder_stm(vec![], Box::new(|_| MOVE(Box::new(a), Box::new(b))))
        },
        EXP(boxed_exp) => match *boxed_exp {
            CALL(function_label, args) => {
                let mut exps = args;
                exps.push(*function_label);
                reorder_stm(
                    exps,
                    Box::new(|mut l| EXP(Box::new(CALL(Box::new(l.pop().expect("exp call canonization")), l))))
                )
            },
            e => reorder_stm(vec![e], Box::new(|mut l| EXP(Box::new(l.pop().expect("exp canonization")))))
        }
        s => reorder_stm(vec![], Box::new(|_| s))
    }
}

fn do_exp(exp: Tree::Exp) -> (Tree::Stm, Tree::Exp) {
    match exp {
        BINOP(oper, a, b) =>
            reorder_exp(
                vec![*a, *b],
                Box::new(|mut l| {
                    let b = Box::new(l.pop().expect("binop canonization"));
                    let a = Box::new(l.pop().expect("binop canonization"));
                    BINOP(oper, a, b)
                })
            ),
        MEM(m) =>
            reorder_exp(
                vec![*m],
                Box::new(|mut l| MEM(Box::new(l.pop().expect("mem canonization"))))
            ),
        ESEQ(s, e) => {
            let stms = do_stm(*s);
            let (stms_, exps) = do_exp(*e);
            (seq(stms, stms_), exps)
        },
        CALL(function_label, args) =>{
            let mut exps = args;
            exps.push(*function_label);
            reorder_exp(exps, Box::new(|mut l| CALL(Box::new(l.pop().expect("call canonization")), l)))
        }
        e => reorder_exp(vec![], Box::new(|_| e))
    }
}

pub fn linearize(tree: Tree::Stm) -> Vec<Tree::Stm> {
    fn linear(tree: Tree::Stm, mut list: Vec<Tree::Stm>) -> Vec<Tree::Stm> {
        if let SEQ(a, b) = tree {
            linear(*a, linear(*b, list))
        } else {
            list.insert(0, tree);
            list
        }
    }
    let linear = linear(do_stm(tree), vec![]);
    console_log!("linear: {:?}", &linear);
    linear
}