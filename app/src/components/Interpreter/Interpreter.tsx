import React from 'react';
import { CanonResult } from '../Compiler/CompilerInterface';
import './Interpreter.scss';

const noop = () => {};

interface InterpreterProps {
    canon: CanonResult;
}
export const Interpreter: React.FC<InterpreterProps> = props => {
    const fragmentsAvailable = props.canon !== null;

    return (
        <div className="interpreter-container">
            {fragmentsAvailable && <RunButton onClick={noop} />}
        </div>
    );
};

interface RunButtonProps {
    onClick: () => void;
}

const RunButton: React.FC<RunButtonProps> = props => {
    return (
        <button className="run-button" onClick={props.onClick}>
            Run interpreter
        </button>
    );
};
