import React, { useState } from 'react';
import { CodeEditor } from '../CodeEditor/CodeEditor';
import { ASTViewer } from '../ASTViewer/ASTViewer';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { Tabs } from '../Tabs/Tabs';
import { TREEViewer } from '../TREEViewer/TREEViewer';

const baseCode = `
/* Enter your tiger code */
0
`;

// Bad type. You can get either Ok or Err. Improve this.
export type RustOption<T> = {
    Ok?: T;
    Err?: any;
};

export type ParseResult = RustOption<any> | null;
export type TypecheckResult = RustOption<any> | null;
export type EscapeResult = any; // Why is this not an option?
export type TranslateResult = RustOption<any> | null;
export type CanonResult = any; // Not implemented
export type WasmResult = any; // Not implemented

interface CompilerProps {
    compile(
        source: string
    ): {
        parse: ParseResult;
        typecheck: TypecheckResult;
        escape: EscapeResult;
        translate: TranslateResult;
        canon: CanonResult;
        wasm: WasmResult;
    };
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [code, setCode] = useLocalStorageState<string>(
        'hector-code',
        baseCode
    );
    const [ast, setAst] = useState<EscapeResult>(null);
    const [fragments, setFragments] = useState<TranslateResult>(null);

    const onClick = () => {
        const result = compile(code);
        console.log(result);
        setAst(result.escape);
        setFragments(result.translate);
    };

    // THIS IS SHIT. MAKE IT RIGHT. COMPOSE COMPONENTS LIKE YOU'RE SUPPOSED TO.
    const tabs = {
        Editor: <CodeEditor code={code} setCode={setCode} />,
        AST: <ASTViewer ast={ast} />,
        TREE: <TREEViewer fragments={fragments} />,
        Result: <p>not implemented</p>,
    };

    return (
        <>
            <div className="compiler-interface">
                <Tabs tabs={tabs} />
            </div>
            <button onClick={onClick}>Compile</button>
        </>
    );
};
