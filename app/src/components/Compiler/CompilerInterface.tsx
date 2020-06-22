import React, { useMemo } from 'react';
import { CodeEditor } from '../CodeEditor/CodeEditor';
import { ASTViewer } from '../ASTViewer/ASTViewer';
import { TREEViewer } from '../TREEViewer/TREEViewer';
import { CanonViewer } from '../CanonViewer/CanonViewer';
import { Interpreter } from '../Interpreter/Interpreter';
import { Frag } from '../../interpreter/treeTypes';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { baseCode } from '../../utils/baseCode';
import { Evaluator } from '../Evaluator/Evaluator';
import { Tab } from '../Tabs/Tab';
import { Tabs } from '../Tabs/Tabs';

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
    wasm: string | null;
    bin: any;
}

interface CompilerProps {
    compile(source: string): CompileResult;
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [compiledCode, setCompiledCode] = useLocalStorageState<string>('hector-code', baseCode);

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
                bin: null,
            };
        }
    }, [compile, compiledCode]);

    return (
        <div className="compiler-interface">
            <Tabs>
                <Tab name="Editor">
                    <CodeEditor compileCode={setCompiledCode} />
                </Tab>
                <Tab name="AST">
                    <ASTViewer ast={compileResult.escape} />
                </Tab>
                <Tab name="TREE">
                    <TREEViewer fragments={compileResult.translate} />
                </Tab>
                <Tab name="Canon">
                    <CanonViewer canon={compileResult.canon} />
                </Tab>
                <Tab name="Interpreter">
                    <Interpreter canon={compileResult.canon} />
                </Tab>
                <Tab name="Evaluator">
                    <Evaluator bin={compileResult.bin} />
                </Tab>
            </Tabs>
            <p className="compile-instructions">
                Psst! compile the code with <strong>Ctrl + enter</strong> or{' '}
                <strong>Ctrl + s</strong>
            </p>
        </div>
    );
};
