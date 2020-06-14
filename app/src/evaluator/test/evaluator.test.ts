import { evaluatorDependenciesFactory } from './testUtils';
import { testExpectedValues } from '../../utils/utils';

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
