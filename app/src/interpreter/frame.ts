import { Exp } from './treeTypes';

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
