import React from 'react';
import ReactJson from 'react-json-view';
import { CanonResult } from '../Compiler/CompilerInterface';

interface CanonViewerProps {
    canon: CanonResult | null;
}

export const CanonViewer: React.FC<CanonViewerProps> = (props) => {
    const { canon } = props;

    if (canon === null) {
        return (
            <div>
                <h3>No fragments to show!</h3>
                <p>Did you compile the code?</p>
                <p>If so, check the WASM results to see if there were any errors</p>
            </div>
        );
    } else {
        return <OkCanonViewer canon={canon} />;
    }
};

export const OkCanonViewer: React.FC<CanonViewerProps> = (props) => {
    // We're sure this is NOT null
    const { canon } = props;

    return (
        <div className="canon-viewer">
            <ReactJson
                src={canon as any}
                theme="monokai"
                enableClipboard={false}
                displayObjectSize={false}
                displayDataTypes={false}
            />
        </div>
    );
};
