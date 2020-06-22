import { UserConsole } from './useConsole';
import { Frag } from '../interpreter/treeTypes';
import { TreeInterpreter } from '../interpreter/interpreter';
import { useMemo, useState, useCallback } from 'react';
import { OutOfBoundsException, NilPointerException } from '../utils/runtimeUtils';

export type RunFunction = () => Promise<void>;

export const useInterpreter = (
    customConsole: UserConsole,
    frags: Frag[]
): [RunFunction, boolean] => {
    const [isRunning, setIsRunning] = useState<boolean>(false);

    const interpreter = useMemo(() => new TreeInterpreter(frags, customConsole), [
        frags,
        customConsole,
    ]);

    const run = useCallback(async () => {
        customConsole.clear();
        setIsRunning(true);
        try {
            const result = await interpreter.run();
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
    }, [interpreter, customConsole]);

    return [run, isRunning];
};
