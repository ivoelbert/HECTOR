import React from 'react';
import { CanonResult } from '../Compiler/CompilerInterface';
import './Interpreter.scss'

interface InterpreterProps {
    canon: CanonResult;
}
export const Interpreter: React.FC<InterpreterProps> = props => {
    return (
        <div className="interpreter-container">
            <p>Not implemented... YET!</p>
        </div>
    )
};
