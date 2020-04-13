import React, { useMemo } from 'react';
import { CodeEditor } from '../CodeEditor/CodeEditor';
import { ASTViewer } from '../ASTViewer/ASTViewer';
import { Tabs } from '../Tabs/Tabs';
import { TREEViewer } from '../TREEViewer/TREEViewer';
import { CanonViewer } from '../CanonViewer/CanonViewer';
import { Interpreter } from '../Interpreter/Interpreter';
import { Frag } from '../../interpreter/treeTypes';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { baseCode } from '../../utils/baseCode';

// Bad type. You can get either Ok or Err. Improve this.
export type RustOption<T> = {
    Ok?: T;
    Err?: any;
};

export type ParseResult = RustOption<any> | null;
export type TypecheckResult = RustOption<any> | null;
export type EscapeResult = any;
export type TranslateResult = RustOption<any> | null;
export type CanonResult = Frag[];
export type WasmResult = any; // Not implemented

interface CompileResult {
    parse: ParseResult;
    typecheck: ParseResult;
    escape: any;
    translate: ParseResult;
    canon: CanonResult | null;
    wasm: any;
}

interface CompilerProps {
    compile(source: string): CompileResult;
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [compiledCode, setCompiledCode] = useLocalStorageState<string>(
        'hector-code',
        baseCode
    );

    const compileResult: CompileResult = useMemo(() => {
        try {
            const result = compile(compiledCode);
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
            };
        }
    }, [compile, compiledCode]);

    // I should make this into a nicer component
    const tabs = {
        Editor: <CodeEditor compileCode={setCompiledCode} />,
        AST: <ASTViewer ast={compileResult.escape} />,
        TREE: <TREEViewer fragments={compileResult.translate} />,
        Canon: <CanonViewer canon={compileResult.canon} />,
        Interp: <Interpreter canon={compileResult.canon} />,
        Result: <p>not implemented</p>,
    };

    return (
        <div className="compiler-interface">
            <Tabs tabs={tabs} />
            <p className="compile-instructions">
                Psst! compile the code with <strong>Ctrl + enter</strong> or{' '}
                <strong>Ctrl + s</strong>
            </p>
        </div>
    );
};
