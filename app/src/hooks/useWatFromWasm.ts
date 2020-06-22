import binaryen from 'binaryen';
import { useMemo } from 'react';

export const useWatFromWasm = (bin: any): string => {
    const memoizedText = useMemo(() => {
        const wasmModule = binaryen.readBinary(bin);
        return wasmModule.emitText();
    }, [bin]);

    return memoizedText;
};
