import React from 'react';
import { EXAMPLES } from './examplesMap';
import { CompileCodeAction } from '../../hooks/useCompileResult';

interface ExamplesProps {
    setCode: React.Dispatch<React.SetStateAction<string>>;
    compileCode: CompileCodeAction;
}

export const Examples: React.FC<ExamplesProps> = ({ setCode, compileCode }) => {
    return (
        <div className="examples-list-container">
            <h3>Examples</h3>
            <ul className="examples-list">
                {Object.entries(EXAMPLES).map(([fileName, content]) => {
                    return (
                        <ExampleItem
                            key={fileName}
                            fileName={fileName}
                            content={content}
                            setCode={setCode}
                            compileCode={compileCode}
                        />
                    );
                })}
            </ul>
        </div>
    );
};

interface ExampleItemProps {
    fileName: string;
    content: string;
    setCode: React.Dispatch<React.SetStateAction<string>>;
    compileCode: CompileCodeAction;
}
const ExampleItem: React.FC<ExampleItemProps> = ({ fileName, content, setCode, compileCode }) => {
    const onClick = () => {
        setCode(content);
        compileCode(content);
    };

    return (
        <li>
            <button onClick={onClick}>{fileName}</button>
        </li>
    );
};
