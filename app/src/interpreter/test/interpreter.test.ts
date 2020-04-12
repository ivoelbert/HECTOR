import { interpreterDependenciesFactory } from './testUtils';
import { returnNumberTestInput } from './inputs/returnNumber';
import { returnVariableTestInput } from './inputs/returnVariable';
import { callIdentityInput } from './inputs/callIdentity';
import { callFactorialInput } from './inputs/callFactorial';
import { addoneTestInput } from './inputs/callAddone';
import { returnArrayElementInput } from './inputs/returnArrayElement';
import { returnRecordElementTestInput } from './inputs/returnRecordElement';
import { escapeInput } from './inputs/escape';
import { mergeInput } from './inputs/merge';
import { queensInput } from './inputs/queens';
import { basicForInput } from './inputs/basicFor';
import { localHideGlobalInput } from './inputs/localHideGlobal';
import { complexStructInput } from './inputs/complexStruct';
import { nilRecordInput } from './inputs/nilRecord';
import { whileWithBreakInput } from './inputs/whileWithBreak';
import { basicWhileInput } from './inputs/basicWhile';
import { printGetcharInput } from './inputs/printGetchar';
import { printPerroInput } from './inputs/printPerro';

test('Basic program that returns 42 works', async () => {
    const { interpreter } = interpreterDependenciesFactory(returnNumberTestInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Define and return variable works', async () => {
    const { interpreter } = interpreterDependenciesFactory(returnVariableTestInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Calling the identity function works', async () => {
    const { interpreter } = interpreterDependenciesFactory(callIdentityInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Calling the factorial function works', async () => {
    const { interpreter } = interpreterDependenciesFactory(callFactorialInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(120);
});

test('Calling the addone function works', async () => {
    const { interpreter } = interpreterDependenciesFactory(addoneTestInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Return the Nth element of an array', async () => {
    const { interpreter } = interpreterDependenciesFactory(returnArrayElementInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Return a field of a record', async () => {
    const { interpreter } = interpreterDependenciesFactory(returnRecordElementTestInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(42);
});

test('Escaped variables work', async () => {
    const { interpreter } = interpreterDependenciesFactory(escapeInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(4);
});

// test('Mergesort works', async () => {
//     const { interpreter } = interpreterDependenciesFactory(mergeInput);
//     const returnValue = await interpreter.run();

//     expect(returnValue).toBe(0);
// });

// test('Queens works', async () => {
//     const { interpreter } = interpreterDependenciesFactory(queensInput);
//     const returnValue = await interpreter.run();

//     expect(returnValue).toBe(0);
// });

test('Basic for loop works', async () => {
    const { interpreter } = interpreterDependenciesFactory(basicForInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(55);
});

test('Local variable hides a global', async () => {
    const { interpreter } = interpreterDependenciesFactory(localHideGlobalInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(2);
});

test('Complex structures work', async () => {
    const { interpreter } = interpreterDependenciesFactory(complexStructInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(0);
});

test('Assign nil to a record', async () => {
    const { interpreter } = interpreterDependenciesFactory(nilRecordInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(0);
});

test('While with break works', async () => {
    const { interpreter } = interpreterDependenciesFactory(whileWithBreakInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(10);
});

test('Basic while works', async () => {
    const { interpreter } = interpreterDependenciesFactory(basicWhileInput);
    const returnValue = await interpreter.run();

    expect(returnValue).toBe(10);
});

test('Print getchar input', async () => {
    const { interpreter, customConsole } = interpreterDependenciesFactory(printGetcharInput);

    const stringToRead = 'perro';
    customConsole.setReadResult('perro');
    const returnValue = await interpreter.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe(stringToRead);
    expect(returnValue).toBe(0);
});

test('Print perro works', async () => {
    const { interpreter, customConsole } = interpreterDependenciesFactory(printPerroInput);

    const returnValue = await interpreter.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe('perro');
    expect(returnValue).toBe(0);
});
