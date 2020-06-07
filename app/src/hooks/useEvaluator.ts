import { InterpConsole } from './useConsole';
import { useMemo, useState, useCallback } from 'react';
import { WasmEvaluator } from '../evaluator/evaluator';

export type RunFunction = () => Promise<void>;

export const useEvaluator = (customConsole: InterpConsole, bin: any): [RunFunction, boolean] => {
    const [isRunning, setIsRunning] = useState<boolean>(false);

    const evaluator = useMemo(() => new WasmEvaluator(Uint8Array.from(bin), customConsole), [
        bin,
        customConsole,
    ]);

    const run = useCallback(async () => {
        customConsole.clear();
        setIsRunning(true);
        try {
            debugger;
            const result = await evaluator.run();
            customConsole.printLine(`Program ended returning ${result}`);
        } catch (err) {
            console.error(err);
            customConsole.printLine('Program failed! Check the console for further details');
        }
        setIsRunning(false);
    }, [evaluator, customConsole]);

    return [run, isRunning];
};
