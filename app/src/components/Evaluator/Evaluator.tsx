import React from 'react';
import { EvaluatorOk } from './EvaluatorOk';
import { ErrorBoundary } from './ErrorBoundary';
import './Evaluator.scss';

interface EvaluatorProps {
    bin: any;
}

export const Evaluator: React.FC<EvaluatorProps> = (props) => {
    if (props.bin === null) {
        return <EvaluatorError />;
    } else {
        return (
            <ErrorBoundary fallback={<EvaluatorError />}>
                <EvaluatorOk bin={props.bin} />
            </ErrorBoundary>
        );
    }
};

const EvaluatorError: React.FC = () => {
    return (
        <div className="evaluator-container">
            <h3>Something went wrong compiling the code!</h3>
            <p>Check the console for further info</p>
        </div>
    );
};
