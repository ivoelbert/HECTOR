import { MockConsole } from './mockConsole';
import { Runtime } from '../src/interpreter/runtime';
import { MemMap } from '../src/interpreter/utils/memMap';
import { Label } from '../src/interpreter/treeTypes';
import { StringStorage } from '../src/interpreter/utils/stringStorage';
import { TreeInterpreter } from '../src/interpreter/interpreter';
import { promises as fs } from 'fs';

interface RuntimeTestDependencies {
    memMap: MemMap;
    labels: Map<Label, number>;
    stringStorage: StringStorage;
    customConsole: MockConsole;
    runtime: Runtime;
}

export const runtimeDependenciesFactory = (): RuntimeTestDependencies => {
    const memMap = new MemMap();
    const labels = new Map<Label, number>();
    const stringStorage = new StringStorage(memMap, labels);
    const customConsole = new MockConsole();
    const runtime = new Runtime(memMap, stringStorage, customConsole);

    return {
        memMap,
        labels,
        stringStorage,
        customConsole,
        runtime,
    };
};

interface InterpreterTestDependencies {
    interpreter: TreeInterpreter;
    customConsole: MockConsole;
}

export const interpreterDependenciesFactory = async (
    file: string
): Promise<InterpreterTestDependencies> => {
    const baseName = file.split('.tig')[0];

    const inputPath = `test/inputs/canon/${baseName}.json`;
    const rawJson = await fs.readFile(inputPath, 'utf8');
    const canon = JSON.parse(rawJson);

    const customConsole = new MockConsole();
    const interpreter = new TreeInterpreter(canon, customConsole);

    return {
        interpreter,
        customConsole,
    };
};
