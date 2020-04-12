import { Stm, LabelStm, ExpStm, MoveStm, JumpStm, CjumpStm, SeqStm } from '../treeTypes';

export const isExpStm = (stm: Stm): stm is ExpStm => {
    return 'EXP' in stm;
};

export const isMoveStm = (stm: Stm): stm is MoveStm => {
    return 'MOVE' in stm;
};

export const isJumpStm = (stm: Stm): stm is JumpStm => {
    return 'JUMP' in stm;
};

export const isCjumpStm = (stm: Stm): stm is CjumpStm => {
    return 'CJUMP' in stm;
};

export const isSeqStm = (stm: Stm): stm is SeqStm => {
    return 'SEQ' in stm;
};

export const isLabelStm = (stm: Stm): stm is LabelStm => {
    return 'LABEL' in stm;
};
