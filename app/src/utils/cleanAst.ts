export interface CleanOptions {
    cleanType?: boolean;
    cleanPosition?: boolean;
    cleanNode?: boolean;
    cleanEscape?: boolean;
}

export const cleanAst = (ast: any, options: CleanOptions = {}): any => {
    const {
        cleanType = true,
        cleanPosition = true,
        cleanNode = true,
        cleanEscape = true,
    } = options;

    // If it's a json
    if (typeof ast === 'object' && !Array.isArray(ast) && ast !== null) {
        const prettyAst: any = {};

        Object.keys(ast).forEach((k) => {
            // Clean nodes if necessary
            if (k === 'node' && cleanNode) {
                const nodeName = Object.keys(ast['node'])[0];
                prettyAst[nodeName] = cleanAst(ast['node'][nodeName], options);
                return;
            }

            // Clean position if necessary
            if (k === 'pos' && cleanPosition) {
                return;
            }

            // Clean type if necessary
            if (k === 'typ' && cleanType) {
                return;
            }

            // Clean escapes if necessary
            if (k === 'escape' && cleanEscape) {
                return;
            }

            prettyAst[k] = cleanAst(ast[k], options);
        });

        return prettyAst;
    }

    // If it's an array
    if (typeof ast === 'object' && Array.isArray(ast)) {
        return ast.map((elem) => cleanAst(elem, options));
    }

    // If it's not an object/array don't modify it.
    return ast;
};
