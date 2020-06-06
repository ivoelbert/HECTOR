import { MockConsole } from './mockConsole';
import { Runtime } from '../runtime';
import { MemMap } from '../utils/memMap';
import { Label } from '../treeTypes';
import { StringStorage } from '../utils/stringStorage';
import { TreeInterpreter } from '../interpreter';
import { promises as fs } from 'fs';
import { ExpectedValues } from '../../utils/expectedValues';

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
    expectedValues: ExpectedValues;
}

export const interpreterDependenciesFactory = async (
    file: string
): Promise<InterpreterTestDependencies> => {
    const baseName = file.split('.tig')[0];

    const canonPath = `test/inputs/canon/${baseName}.json`;
    const expectedValuesPath = `test/inputs/expectedValues/${baseName}.json`;
    const [rawCanon, rawExpectedValues] = await Promise.all([
        fs.readFile(canonPath, 'utf8'),
        fs.readFile(expectedValuesPath, 'utf8'),
    ]);

    const canon = JSON.parse(rawCanon);
    const expectedValues = JSON.parse(rawExpectedValues);

    const customConsole = new MockConsole();
    const interpreter = new TreeInterpreter(canon, customConsole);

    return {
        interpreter,
        customConsole,
        expectedValues,
    };
};
