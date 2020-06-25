import React from 'react';
import { useExamples } from './useExamples';

interface ExamplesProps {
    setCode: React.Dispatch<React.SetStateAction<string>>;
}

export const Examples: React.FC<ExamplesProps> = ({ setCode }) => {
    const [examples] = useExamples();

    return (
        <div className="examples-list-container">
            <h3>Examples</h3>
            <ul className="examples-list">
                {Object.entries(examples).map(([fileName, content]) => {
                    return (
                        <ExampleItem
                            key={fileName}
                            fileName={fileName}
                            content={content}
                            setCode={setCode}
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
}
const ExampleItem: React.FC<ExampleItemProps> = ({ fileName, content, setCode }) => {
    const onClick = () => {
        setCode(content);
    };

    return (
        <li>
            <button onClick={onClick}>{fileName}</button>
        </li>
    );
};
