import { CompileResult, CompileFunction } from '../components/Compiler/CompilerInterface';
import { useMemo } from 'react';

export const useCompileResult = (code: string, compile: CompileFunction): CompileResult => {
    const compileResult: CompileResult = useMemo(() => {
        try {
            const result = compile(code);
            console.log(result);
            return result;
        } catch (err) {
            console.log('Something went wrong compiling your code!');
            console.error(err);
            return {
                parse: null,
                typecheck: null,
                escape: null,
                translate: null,
                canon: null,
                wasm: null,
                bin: null,
            };
        }
    }, [compile, code]);

    return compileResult;
};
