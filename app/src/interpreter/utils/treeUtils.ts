import { Label, Stm, BinOp } from '../treeTypes';
import { isLabelStm } from './stmPatterns';
import { UnreachableError } from './utils';

export const findLabelIndex = (stms: Stm[], label: Label): number => {
    const foundIndex = stms.findIndex((stm) => {
        return isLabelStm(stm) && stm.LABEL === label;
    });

    if (foundIndex === -1) {
        throw new Error(`Could not find label '${label}'!`);
    }

    return foundIndex;
};

export const evalBinop = (op: BinOp, leftVal: number, rightVal: number): number => {
    switch (op) {
        case 'PLUS':
            return leftVal + rightVal;

        case 'MINUS':
            return leftVal - rightVal;

        case 'MUL':
            return leftVal * rightVal;

        case 'DIV':
            return Math.floor(leftVal / rightVal);

        case 'AND':
            return leftVal && rightVal;

        case 'OR':
            return leftVal || rightVal;

        case 'LSHIFT':
            return leftVal << rightVal;

        case 'RSHIFT':
            return leftVal >>> rightVal;

        case 'ARSHIFT':
            return leftVal >> rightVal;

        case 'XOR':
            return leftVal ^ rightVal;

        case 'EQ':
            return Number(leftVal === rightVal);

        case 'NE':
            return Number(leftVal !== rightVal);

        case 'LT':
            return Number(leftVal < rightVal);

        case 'GT':
            return Number(leftVal > rightVal);

        case 'LE':
            return Number(leftVal <= rightVal);

        case 'GE':
            return Number(leftVal >= rightVal);

        case 'ULT':
            // Not sure, our parser doesn't even generate this
            return Number(leftVal < rightVal);

        case 'ULE':
            // Not sure, our parser doesn't even generate this
            return Number(leftVal <= rightVal);

        case 'UGT':
            // Not sure, our parser doesn't even generate this
            return Number(leftVal > rightVal);

        case 'UGE':
            // Not sure, our parser doesn't even generate this
            return Number(leftVal >= rightVal);

        default:
            throw new UnreachableError();
    }
};
