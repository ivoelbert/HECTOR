import { UserConsole } from './useConsole';
import { useMemo, useState, useCallback } from 'react';
import { WasmEvaluator } from '../evaluator/evaluator';
import { OutOfBoundsException, NilPointerException } from '../utils/runtimeUtils';

export type RunFunction = () => Promise<void>;

export const useEvaluator = (customConsole: UserConsole, bin: any): [RunFunction, boolean] => {
    const [isRunning, setIsRunning] = useState<boolean>(false);

    const evaluator = useMemo(() => new WasmEvaluator(Uint8Array.from(bin), customConsole), [
        bin,
        customConsole,
    ]);

    const run = useCallback(async () => {
        customConsole.clear();
        setIsRunning(true);
        try {
            const result = await evaluator.run();
            customConsole.printLine(`Program ended returning ${result}`);
        } catch (err) {
            if (err instanceof OutOfBoundsException) {
                customConsole.print(
                    `Program failed!\nArray index out of bounds.\nCannot access index ${err.index} from pointer ${err.pointer}`
                );
            } else if (err instanceof NilPointerException) {
                customConsole.print(
                    `Program failed!\nNil record exception.\nCannot access a field of a nil record`
                );
            } else {
                console.error(err);
                customConsole.printLine('Program failed! Check the console for further details');
            }
        }
        setIsRunning(false);
    }, [evaluator, customConsole]);

    return [run, isRunning];
};
