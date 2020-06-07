import React from 'react';
import { CustomConsole } from '../CustomConsole/CustomConsole';
import { useConsole } from '../../hooks/useConsole';
import { useEvaluator } from '../../hooks/useEvaluator';

interface EvaluatorOkProps {
    bin: any;
}

export const EvaluatorOk: React.FC<EvaluatorOkProps> = (props) => {
    const { bin } = props;
    const [customConsole, messages, isReading] = useConsole();

    const [run, isRunning] = useEvaluator(customConsole, bin);

    return (
        <div className="evaluator-container">
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
        <button className="run-button" disabled={props.disabled} onClick={props.onClick}>
            Run interpreter
        </button>
    );
};
