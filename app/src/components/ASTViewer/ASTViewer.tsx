import React, { useMemo, useState } from 'react';
import ReactJson from 'react-json-view';
import { cleanAst, CleanOptions } from '../../utils/cleanAst';
import { EscapeResult } from '../Compiler/CompilerInterface';
import './ASTViewer.scss';

interface ASTViewerProps {
    ast: EscapeResult;
}

export const ASTViewer: React.FC<ASTViewerProps> = (props) => {
    const { ast } = props;

    if (ast === null) {
        return (
            <div>
                <h3>No AST to show!</h3>
                <p>Did you compile the code?</p>
                <p>If so, check the WASM results to see if there were any errors</p>
            </div>
        );
    } else {
        return <OkASTViewer ast={ast} />;
    }
};

const OkASTViewer: React.FC<ASTViewerProps> = (props) => {
    const { ast } = props;

    const [options, setOptions] = useState<CleanOptions>({
        cleanType: true,
        cleanPosition: true,
        cleanNode: true,
        cleanEscape: true,
    });

    const prettyAst = useMemo(() => cleanAst(ast, options), [ast, options]);

    // An AST is valid if it has at least one key
    const isValidAst = Object.keys(prettyAst).length > 0;

    return (
        <div className="ast-viewer">
            <ReactJson
                src={prettyAst}
                theme="monokai"
                enableClipboard={false}
                displayObjectSize={false}
                displayDataTypes={false}
            />
            {isValidAst && <CleanControls options={options} setOptions={setOptions} />}
        </div>
    );
};

interface CleanControlsProps {
    options: CleanOptions;
    setOptions: React.Dispatch<React.SetStateAction<CleanOptions>>;
}

const CleanControls: React.FC<CleanControlsProps> = ({ options, setOptions }) => {
    const [collapsed, setCollapsed] = useState<boolean>(true);
    const toggleCollapse = () => setCollapsed(!collapsed);

    const optionsClassName = `options-container ${collapsed ? 'collapsed' : 'expanded'}`;

    return (
        <div className="clean-controls-container">
            <button className="collapse-button" onClick={toggleCollapse}>
                {collapsed ? 'Options' : 'Collapse'}
            </button>
            <div className={optionsClassName}>
                {Object.entries(options).map(([option, value]) => {
                    const onChange = () => setOptions({ ...options, [option]: !value });
                    return (
                        <div className="option-checkbox" key={option}>
                            <input
                                id={option}
                                type="checkbox"
                                checked={value}
                                onChange={onChange}
                            />
                            <label htmlFor={option}>{option}</label>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};
