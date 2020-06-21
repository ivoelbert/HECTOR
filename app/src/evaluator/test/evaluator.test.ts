import { evaluatorDependenciesFactory } from './testUtils';
import { testExpectedValues } from '../../utils/utils';
import { OutOfBoundsException, NilPointerException } from '../../utils/runtimeUtils';

test('returnNumber program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('returnNumber.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('returnVariable program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('returnVariable.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('callIdentity program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('callIdentity.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('callFactorial program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('callFactorial.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('callAddone program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('callAddone.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('escape program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('escape.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('escape2 program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('escape2.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('basicFor program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('basicFor.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('localHideGlobal program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('localHideGlobal.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('whileWithBreak program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('whileWithBreak.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('basicWhile program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('basicWhile.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('returnArrayElement program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory(
        'returnArrayElement.tig'
    );
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('returnArrayInit program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('returnArrayInit.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('printPerro program works', async () => {
    const { evaluator, expectedValues, customConsole } = await evaluatorDependenciesFactory(
        'printPerro.tig'
    );
    const returnValue = await evaluator.run();

    const lastMessage = customConsole.getLastMessage();
    expect(lastMessage).toBe('perro');

    testExpectedValues(returnValue, expectedValues);
});

test('stringSize program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('stringSize.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('stringSlice program works', async () => {
    const { evaluator, expectedValues, customConsole } = await evaluatorDependenciesFactory(
        'stringSlice.tig'
    );
    const returnValue = await evaluator.run();

    const lastMessage = customConsole.getLastMessage();
    expect(lastMessage).toBe('perro');

    testExpectedValues(returnValue, expectedValues);
});

test('concatStrings program works', async () => {
    const { evaluator, expectedValues, customConsole } = await evaluatorDependenciesFactory(
        'concatStrings.tig'
    );
    const returnValue = await evaluator.run();

    const lastMessage = customConsole.getLastMessage();
    expect(lastMessage).toBe('milanesa');

    testExpectedValues(returnValue, expectedValues);
});

test('stringCompare program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('stringCompare.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('returnRecordElement program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory(
        'returnRecordElement.tig'
    );
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('earlyExit program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('earlyExit.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('printGetchar program works', async () => {
    const { evaluator, expectedValues, customConsole } = await evaluatorDependenciesFactory(
        'printGetchar.tig'
    );

    const stringToRead = 'x';
    customConsole.setReadResult(stringToRead);
    const returnValue = await evaluator.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe(stringToRead);
    testExpectedValues(returnValue, expectedValues);
});

test('printGetstring program works', async () => {
    const { evaluator, expectedValues, customConsole } = await evaluatorDependenciesFactory(
        'printGetstring.tig'
    );

    const stringToRead = 'perro';
    customConsole.setReadResult(stringToRead);
    const returnValue = await evaluator.run();
    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe(stringToRead);
    testExpectedValues(returnValue, expectedValues);
});

test('indexOutOfBounds throws the correct error', async () => {
    const { evaluator } = await evaluatorDependenciesFactory('indexOutOfBounds.tig');

    try {
        await evaluator.run();
        fail('Out of bounds index should throw an OutOfBoundsError');
    } catch (err) {
        expect(err).toBeInstanceOf(OutOfBoundsException);
    }
});

test('nilRecordError throws the correct error', async () => {
    const { evaluator } = await evaluatorDependenciesFactory('nilRecordError.tig');

    try {
        await evaluator.run();
        fail('nil record access field should throw a NilPointerException');
    } catch (err) {
        expect(err).toBeInstanceOf(NilPointerException);
    }
});
