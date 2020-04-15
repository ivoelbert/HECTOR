import { interpreterDependenciesFactory } from './testUtils';

test('Basic program that returns 42 works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'returnNumber.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Define and return variable works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'returnVariable.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Calling the identity function works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'callIdentity.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Calling the factorial function works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'callFactorial.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(120);
});

test('Calling the addone function works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'callAddone.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Return the Nth element of an array', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'returnArrayElement.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Return a field of a record', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'returnRecordElement.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Escaped variables work', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'escape.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(4);
});

test('Queens works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'queens.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(0);
});

test('Basic for loop works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'basicFor.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(55);
});

test('Local variable hides a global', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'localHideGlobal.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(2);
});

test('Complex structures work', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'complexStruct.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(0);
});

test('Assign nil to a record', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'nilRecord.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(0);
});

test('While with break works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'whileWithBreak.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(10);
});

test('Basic while works', async () => {
    const { interpreter } = await interpreterDependenciesFactory(
        'basicWhile.tig'
    );
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(10);
});

test('Print getchar input', async () => {
    const { interpreter, customConsole } = await interpreterDependenciesFactory(
        'printGetchar.tig'
    );

    const stringToRead = 'perro';
    customConsole.setReadResult('perro');
    const returnValue = await interpreter.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe(stringToRead);
    expect(returnValue).toBe(0);
});

test('Print perro works', async () => {
    const { interpreter, customConsole } = await interpreterDependenciesFactory(
        'printPerro.tig'
    );

    const returnValue = await interpreter.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe('perro');
    expect(returnValue).toBe(0);
});
