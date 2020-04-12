import React from 'react';
import { CanonResult } from '../Compiler/CompilerInterface';
import { InterpreterOk } from './InterpreterOk';

import './Interpreter.scss';

interface InterpreterProps {
    canon: CanonResult | null;
}
export const Interpreter: React.FC<InterpreterProps> = (props) => {
    if (props.canon === null) {
        return <InterpreterErr />;
    } else {
        return <InterpreterOk frags={props.canon} />;
    }
};

const InterpreterErr: React.FC = () => {
    return (
        <div className="interpreter-container">
            <h3>No fragments to show!</h3>
            <p>Did you compile the code?</p>
            <p>If so, check the WASM results to see if there were any errors</p>
        </div>
    );
};
