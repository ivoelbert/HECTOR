import { UserConsole } from './useConsole';
import { Frag } from '../interpreter/treeTypes';
import { TreeInterpreter } from '../interpreter/interpreter';
import { useMemo, useState, useCallback } from 'react';

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
            console.error(err);
            customConsole.printLine('Program failed! Check the console for further details');
        }
        setIsRunning(false);
    }, [interpreter, customConsole]);

    return [run, isRunning];
};
