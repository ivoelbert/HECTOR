import React, { useState } from 'react';
import { CodeEditor } from './CodeEditor';
import { ASTViewer } from './ASTViewer';
import { useLocalStorageState } from '../hooks/useLocalStorageState';
import { Tabs } from './Tabs/Tabs';

const baseCode = `
/* Enter your tiger code */
0
`;

interface CompilerProps {
    compile(
        source: string
    ): {
        parse: any;
        typecheck: any;
        escape: any;
        translate: any;
        canon: any;
        wasm: any;
    };
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [code, setCode] = useLocalStorageState<string>('hector-code', baseCode);
    const [ast, setAst] = useState({});
    const [fragments, setFragments] = useState({});

    const onClick = () => {
        const result = compile(code);
        console.log();
        setAst(result.escape);
        setFragments(result.translate);
    };

    const tabs = {
        Editor: <CodeEditor code={code} setCode={setCode} />,
        AST: <ASTViewer ast={ast} />,
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
