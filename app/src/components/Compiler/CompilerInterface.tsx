import React from 'react';
import { CodeEditor } from '../CodeEditor/CodeEditor';
import { ASTViewer } from '../ASTViewer/ASTViewer';
import { TREEViewer } from '../TREEViewer/TREEViewer';
import { CanonViewer } from '../CanonViewer/CanonViewer';
import { Interpreter } from '../Interpreter/Interpreter';
import { Frag } from '../../interpreter/treeTypes';
import { Evaluator } from '../Evaluator/Evaluator';
import { Tab } from '../Tabs/Tab';
import { Tabs } from '../Tabs/Tabs';
import { WASMViewer } from '../WASMViewer/WASMViewer';
import { useCompileResult } from '../../hooks/useCompileResult';

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

export interface CompileResult {
    parse: ParseResult;
    typecheck: ParseResult;
    escape: any;
    translate: ParseResult;
    canon: CanonResult | null;
    wasm: string | null;
    bin: any;
}

export type CompileFunction = (source: string) => CompileResult;

interface CompilerProps {
    compile: CompileFunction;
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [code, setCode, compileResult, compileCode] = useCompileResult(compile);

    return (
        <div className="compiler-interface">
            <Tabs>
                <Tab name="Editor">
                    <CodeEditor code={code} setCode={setCode} compileCode={compileCode} />
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
                <Tab name="WASM">
                    <WASMViewer bin={compileResult.bin} />
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
