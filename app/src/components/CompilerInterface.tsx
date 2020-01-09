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
    compile(source: string): any;
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [code, setCode] = useLocalStorageState<string>('hector-code', baseCode);
    const [ast, setAst] = useState({});
    const [fragments, setFragments] = useState([]);

    const onClick = () => {
        const [compiledAst, compiledFragments] = compile(code);
        setAst(compiledAst);
        setFragments(compiledFragments);
    };

    const tabs = {
        Editor: <CodeEditor code={code} setCode={setCode} />,
        AST: <ASTViewer ast={ast} />,
        Result: <p>3</p>
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
