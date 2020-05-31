import React, { Suspense } from 'react';
import { CompilerInterface } from './CompilerInterface';

import './Compiler.scss';

const AsyncCompiler = React.lazy(
    async (): Promise<any> => {
        const wasm = await import('hector');
        const Component: React.FC = () => <CompilerInterface compile={wasm.compile} />;

        return { default: Component };
    }
);

export const Compiler: React.FC = () => {
    return (
        <div className="compiler-container">
            <Suspense fallback={<div>Loading...</div>}>
                <AsyncCompiler />
            </Suspense>
        </div>
    );
};
