import { InterpConsole } from './useConsole';
import { Frag } from '../interpreter/treeTypes';
import { TreeInterpreter } from '../interpreter/interpreter';
import { useMemo, useState, useCallback } from 'react';

export type RunFunction = () => Promise<void>;

export const useInterpreter = (
    customConsole: InterpConsole,
    frags: Frag[]
): [RunFunction, boolean] => {
    const [isRunning, setIsRunning] = useState<boolean>(false);

    const interpreter = useMemo(
        () => new TreeInterpreter(frags, customConsole),
        [frags, customConsole]
    );

    const run = useCallback(async () => {
        customConsole.clear();
        setIsRunning(true);
        const result = await interpreter.run();
        customConsole.printLine(`Program ended returning ${result}`);
        setIsRunning(false);
    }, [interpreter, customConsole]);

    return [run, isRunning];
};
