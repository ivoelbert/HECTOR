import React, { Suspense } from 'react';
import { CompilerInterface } from './CompilerInterface';

const AsyncCompiler = React.lazy(
    async (): Promise<any> => {
        const wasm = await import('tigerust');
        const Component: React.FC = () => <CompilerInterface compile={wasm.compile} />;

        return { default: Component };
    }
);

export const Compiler: React.FC = () => {
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <AsyncCompiler />
        </Suspense>
    );
};
