export type Label = string;
export type LocalTemp = string;
export type GlobalTemp = string;

export type ConstExp = { CONST: number };
export type NameExp = { NAME: Label };
export type LocalExp = { LOCAL: LocalTemp };
export type GlobalExp = { GLOBAL: GlobalTemp };
export type BinopExp = { BINOP: [BinOp, Exp, Exp] };
export type MemExp = { MEM: Exp };
export type CallExp = { CALL: [Exp, Exp[]] };
export type EseqExp = { ESEQ: [Stm, Exp] };

export type Exp = ConstExp | NameExp | LocalExp | GlobalExp | BinopExp | MemExp | CallExp | EseqExp;

export type BinOp =
    | 'PLUS'
    | 'MINUS'
    | 'MUL'
    | 'DIV'
    | 'AND'
    | 'OR'
    | 'LSHIFT'
    | 'RSHIFT'
    | 'ARSHIFT'
    | 'XOR'
    | 'EQ'
    | 'NE'
    | 'LT'
    | 'GT'
    | 'LE'
    | 'GE'
    | 'ULT'
    | 'ULE'
    | 'UGT'
    | 'UGE';

export type ExpStm = { EXP: Exp };
export type MoveStm = { MOVE: [Exp, Exp] };
export type JumpStm = { JUMP: [Exp, Label[]] };
export type CjumpStm = { CJUMP: [BinOp, Exp, Exp, Label, Label] };
export type SeqStm = { SEQ: [Stm, Stm] };
export type LabelStm = { LABEL: Label };

export type Stm = ExpStm | MoveStm | JumpStm | CjumpStm | SeqStm | LabelStm;

export type Frame = {
    label: Label;
    formals: [string, boolean][];
    memindex: number;
};

export type FragStm = {
    stms: Stm[];
};

export type FunFrag = {
    Proc: {
        body: FragStm[];
        frame: Frame;
    };
};

export type StringFrag = {
    ConstString: [Label, string];
};

export type Frag = FunFrag | StringFrag;

/*
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Exp {
    CONST(i64),
    NAME(Label),
    TEMP(Temp),
    BINOP(BinOp, Box<Exp>, Box<Exp>),
    MEM(Box<Exp>),
    CALL(Box<Exp>, Vec<Exp>),
    ESEQ(Box<Stm>, Box<Exp>)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Stm {
    EXP(Box<Exp>),
    MOVE(Box<Exp>, Box<Exp>),
    JUMP(Exp, Vec<Label>),
    CJUMP(BinOp, Exp, Exp, Label, Label),
    SEQ(Box<Stm>, Box<Stm>),
    LABEL(Label)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum BinOp {
    PLUS,
    MINUS,
    MUL,
    DIV,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    ARSHIFT,
    XOR,
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
    ULT,
    ULE,
    UGT,
    UGE
}
*/
