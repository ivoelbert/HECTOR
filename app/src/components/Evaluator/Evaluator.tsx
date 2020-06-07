import React from 'react';
import { EvaluatorOk } from './EvaluatorOk';

import './Evaluator.scss';

interface EvaluatorProps {
    bin: any;
}

export const Evaluator: React.FC<EvaluatorProps> = (props) => {
    if (props.bin === null) {
        return <EvaluatorProps />;
    } else {
        return <EvaluatorOk bin={props.bin} />;
    }
};

const EvaluatorProps: React.FC = () => {
    return (
        <div className="evaluator-container">
            <h3>No fragments to show!</h3>
            <p>Did you compile the code?</p>
            <p>If so, check the WASM results to see if there were any errors</p>
        </div>
    );
};
