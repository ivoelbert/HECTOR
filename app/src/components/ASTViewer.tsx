import React, { useMemo, useState } from 'react';
import ReactJson from 'react-json-view';
import { cleanAst, CleanOptions } from '../utils/cleanAst';

interface ASTViewerProps {
    ast: any;
}

export const ASTViewer: React.FC<ASTViewerProps> = props => {
    const { ast } = props;

    // TODO: Interface to control this options
    const [options] = useState<CleanOptions>({
        cleanType: false,
        cleanPosition: true,
        cleanNode: true,
        cleanEscape: true,
    });

    const prettyAst = useMemo(() => cleanAst(ast, options), [ast, options]);

    return (
        <div>
            <ReactJson src={prettyAst} theme="monokai" enableClipboard={false} displayObjectSize={false} displayDataTypes={false} />
        </div>
    );
};
