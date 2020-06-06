import { evaluatorDependenciesFactory } from './testUtils';
import { testExpectedValues } from '../../utils/utils';

test('returnNumber program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('returnNumber.tig');

    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});
