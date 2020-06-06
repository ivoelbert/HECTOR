import { ExpectedValues } from './expectedValues';

export const noop = () => {};

export const testExpectedValues = (returnValue: number, expectedValues: ExpectedValues): void => {
    if (expectedValues.result !== null) {
        expect(returnValue).toBe(expectedValues.result);
    }
};
