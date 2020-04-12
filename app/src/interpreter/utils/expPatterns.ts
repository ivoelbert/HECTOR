import { Exp, MemExp, LocalExp, ConstExp, NameExp, BinopExp, CallExp, EseqExp, GlobalExp } from '../treeTypes';

export const isConstExp = (exp: Exp): exp is ConstExp => {
    return 'CONST' in exp;
};

export const isNameExp = (exp: Exp): exp is NameExp => {
    return 'NAME' in exp;
};

export const isLocalExp = (exp: Exp): exp is LocalExp => {
    return 'LOCAL' in exp;
};

export const isGlobalExp = (exp: Exp): exp is GlobalExp => {
    return 'GLOBAL' in exp;
}

export const isBinopExp = (exp: Exp): exp is BinopExp => {
    return 'BINOP' in exp;
};

export const isMemExp = (exp: Exp): exp is MemExp => {
    return 'MEM' in exp;
};

export const isCallExp = (exp: Exp): exp is CallExp => {
    return 'CALL' in exp;
};

export const isEseqExp = (exp: Exp): exp is EseqExp => {
    return 'ESEQ' in exp;
};
