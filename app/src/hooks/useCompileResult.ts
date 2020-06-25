import { CompileResult, CompileFunction } from '../components/Compiler/CompilerInterface';
import { useState } from 'react';
import { baseCode } from '../utils/baseCode';
import { useLocalStorageState } from './useLocalStorageState';

export type CompileCodeAction = (source: string) => void;

export const useCompileResult = (
    compile: CompileFunction
): [string, React.Dispatch<React.SetStateAction<string>>, CompileResult, CompileCodeAction] => {
    const [code, setCode] = useLocalStorageState<string>('hector-code', baseCode);

    const [compileResult, setCompileResult] = useState<CompileResult>({
        parse: null,
        typecheck: null,
        escape: null,
        translate: null,
        canon: null,
        wasm: null,
        bin: null,
    });

    const [lastCompiledCode, setLastCompiledCode] = useState<string>('');

    const compileCode = (source: string): void => {
        try {
            if (source !== '' && source !== lastCompiledCode) {
                const result = compile(source);
                console.log(result);
                setCompileResult(result);
                setLastCompiledCode(source);
            }
        } catch (err) {
            console.log('Something went wrong compiling your code!');
            console.error(err);
            setCompileResult({
                parse: null,
                typecheck: null,
                escape: null,
                translate: null,
                canon: null,
                wasm: null,
                bin: null,
            });
        }
    };

    return [code, setCode, compileResult, compileCode];
};
