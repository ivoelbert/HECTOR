import { ExpectedValues } from './expectedValues';

export const WORD_SZ = 4;

export const noop = () => {};

export const testExpectedValues = (returnValue: number, expectedValues: ExpectedValues): void => {
    if (expectedValues.result !== null) {
        expect(returnValue).toBe(expectedValues.result);
    }
};

export type nil = null | undefined;
export const isNil = <T>(value: T | nil): value is nil => {
    if (value === null || value === undefined) {
        return true;
    }
    return false;
};
