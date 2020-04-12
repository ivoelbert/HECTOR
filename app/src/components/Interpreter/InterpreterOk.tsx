import React from 'react';
import { CanonResult } from '../Compiler/CompilerInterface';
import { CustomConsole } from '../CustomConsole/CustomConsole';
import { useConsole } from '../../hooks/useConsole';
import { useInterpreter } from '../../hooks/useInterpreter';

interface InterpreterOkProps {
    frags: CanonResult;
}

export const InterpreterOk: React.FC<InterpreterOkProps> = (props) => {
    const { frags } = props;
    const [customConsole, messages, isReading] = useConsole();

    const [run, isRunning] = useInterpreter(customConsole, frags);

    return (
        <div className="interpreter-container">
            <CustomConsole
                customConsole={customConsole}
                messages={messages}
                isReading={isReading}
            />
            <RunButton disabled={isRunning} onClick={run} />
        </div>
    );
};

interface RunButtonProps {
    onClick: () => void;
    disabled: boolean;
}

const RunButton: React.FC<RunButtonProps> = (props) => {
    return (
        <button
            className="run-button"
            disabled={props.disabled}
            onClick={props.onClick}
        >
            Run interpreter
        </button>
    );
};
