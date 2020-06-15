import { Exp } from './treeTypes';

export const WORD_SZ = 4;

const inLocalAccessExp = (name: string): Exp => ({
    LOCAL: name,
});

// For now, all formals go in memory
const accessExpFromFormal = (formal: [string, boolean]): Exp => {
    const [name] = formal;
    return inLocalAccessExp(name);
};

export const accessExpsFromFormals = (formals: [string, boolean][]): Exp[] => {
    return formals.map(accessExpFromFormal);
};
