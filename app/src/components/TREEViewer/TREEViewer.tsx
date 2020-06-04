import React from 'react';
import ReactJson from 'react-json-view';
import { TranslateResult } from '../Compiler/CompilerInterface';

interface TREEViewerProps {
    fragments: TranslateResult;
}

export const TREEViewer: React.FC<TREEViewerProps> = (props) => {
    const { fragments } = props;

    if (fragments === null) {
        return (
            <div>
                <h3>No fragments to show!</h3>
                <p>Did you compile the code?</p>
                <p>If so, check the WASM results to see if there were any errors</p>
            </div>
        );
    } else if (fragments.Err !== undefined) {
        return (
            <div>
                <h3>No fragments to show!</h3>
                <p>check the WASM results, it seems there was an error translating</p>
            </div>
        );
    } else {
        return <OkTREEViewer fragments={fragments} />;
    }
};

export const OkTREEViewer: React.FC<TREEViewerProps> = (props) => {
    // We're sure this is NOT null
    const { fragments } = props;

    return (
        <div className="tree-viewer">
            <ReactJson
                src={fragments as any}
                theme="monokai"
                enableClipboard={false}
                displayObjectSize={false}
                displayDataTypes={false}
            />
        </div>
    );
};
