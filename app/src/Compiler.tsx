import React, { useState } from 'react';
import './App.css';

interface LoadedProps {
    wasm: Wasm;
}
const Loaded: React.FC<LoadedProps> = ({ wasm }) => {
    const onClick = () => {
        console.log(wasm.main('1'));
    };

    return <button onClick={onClick}>Click me</button>;
};

interface UnloadedProps {
    loading: boolean;
    loadWasm: () => Promise<void>;
}
const Unloaded: React.FC<UnloadedProps> = ({ loading, loadWasm }) => {
    return loading ? <div>Loading...</div> : <button onClick={loadWasm}>Load library</button>;
};

interface Wasm {
    main(arg0: string): any;
}

export const Compiler = () => {
    const [loading, setLoading] = useState(false);
    const [wasm, setWasm] = useState<Wasm | null>(null);

    const loadWasm = async () => {
        try {
            setLoading(true);
            const wasm = await import('tigerust');
            setWasm(wasm);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="App">
            <header className="App-header">{wasm ? <Loaded wasm={wasm} /> : <Unloaded loading={loading} loadWasm={loadWasm} />}</header>
        </div>
    );
};
