import React, { useState } from 'react';
import { CodeEditor } from './CodeEditor';
import { ASTViewer } from './ASTViewer';

const baseCode = `
/* Enter your tiger code */
0
`;

interface CompilerProps {
    compile(source: string): any;
}

export const CompilerInterface: React.FC<CompilerProps> = ({ compile }) => {
    const [code, setCode] = useState<string>(baseCode);
    const [ast, setAst] = useState({})

    const onClick = () => {
        setAst(compile(code));
    };

    return (
        <>
            <div className="">
                <CodeEditor code={code} setCode={setCode} />
                <ASTViewer ast={ast} />
            </div>
            <button onClick={onClick}>Compile</button>
        </>
    );
};
