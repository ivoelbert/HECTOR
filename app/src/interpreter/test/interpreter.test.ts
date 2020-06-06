import { interpreterDependenciesFactory } from './testUtils';
import { testExpectedValues } from '../../utils/utils';

test('Basic program that returns 42 works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'returnNumber.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Define and return variable works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'returnVariable.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Calling the identity function works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'callIdentity.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Calling the factorial function works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'callFactorial.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Calling the addone function works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory('callAddone.tig');
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Return the Nth element of an array', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'returnArrayElement.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Return a field of a record', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'returnRecordElement.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Escaped variables work', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory('escape.tig');
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Queens works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory('queens.tig');
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Basic for loop works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory('basicFor.tig');
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Local variable hides a global', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'localHideGlobal.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Complex structures work', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'complexStruct.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Assign nil to a record', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory('nilRecord.tig');
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('While with break works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory(
        'whileWithBreak.tig'
    );
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Basic while works', async () => {
    const { interpreter, expectedValues } = await interpreterDependenciesFactory('basicWhile.tig');
    const returnValue = await interpreter.run();

    testExpectedValues(returnValue, expectedValues);
});

test('Print getchar input', async () => {
    const { interpreter, expectedValues, customConsole } = await interpreterDependenciesFactory(
        'printGetchar.tig'
    );

    const stringToRead = 'perro';
    customConsole.setReadResult('perro');
    const returnValue = await interpreter.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe(stringToRead);
    testExpectedValues(returnValue, expectedValues);
});

test('Print perro works', async () => {
    const { interpreter, expectedValues, customConsole } = await interpreterDependenciesFactory(
        'printPerro.tig'
    );

    const returnValue = await interpreter.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe('perro');
    testExpectedValues(returnValue, expectedValues);
});
