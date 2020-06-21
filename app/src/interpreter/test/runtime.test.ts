import { runtimeDependenciesFactory } from './testUtils';
import { WORD_SZ } from '../../utils/utils';

test('print writes to a mock console', async () => {
    const { stringStorage, customConsole, runtime } = runtimeDependenciesFactory();

    const message = 'some string';
    const stringPtr = stringStorage.storeUnlabeledString(message);
    const printFunction = runtime.getFunction('print');
    await printFunction([stringPtr]);

    const printedMessage = customConsole.getLastMessage();

    expect(printedMessage).toBe(message);
});

test('getstring reads the mocked string', async () => {
    const { stringStorage, customConsole, runtime } = runtimeDependenciesFactory();

    const message = 'some other string';
    customConsole.setReadResult(message);
    const getcharFunction = runtime.getFunction('getstring');
    const strPointer = await getcharFunction([]);

    const readString = stringStorage.loadString(strPointer);

    expect(readString).toBe(message);
});

test('getchar reads the mocked char', async () => {
    const { stringStorage, customConsole, runtime } = runtimeDependenciesFactory();

    const message = 'x';
    customConsole.setReadResult(message);
    const getcharFunction = runtime.getFunction('getchar');
    const strPointer = await getcharFunction([]);

    const readString = stringStorage.loadString(strPointer);

    expect(readString).toBe(message);
});

test('ord returns the right char code', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const character = 'x';
    const stringPtr = stringStorage.storeUnlabeledString(character);
    const ordFunction = runtime.getFunction('ord');
    const charCode = await ordFunction([stringPtr]);

    expect(charCode).toBe(character.charCodeAt(0));
});

test('chr returns the right character', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const charCode = 120;
    const chrFunction = runtime.getFunction('chr');
    const strPointer = await chrFunction([charCode]);
    const readString = stringStorage.loadString(strPointer);

    expect(readString).toBe(String.fromCharCode(charCode));
});

test('size returns the right string length', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const message = 'some string';
    const stringPtr = stringStorage.storeUnlabeledString(message);
    const sizeFunction = runtime.getFunction('size');
    const length = await sizeFunction([stringPtr]);

    expect(length).toBe(message.length);
});

test('substring returns the right string', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const message = 'some string';
    const sliceStart = 2;
    const sliceLength = 5;
    const stringPtr = stringStorage.storeUnlabeledString(message);
    const substringFunction = runtime.getFunction('substring');
    const substringPtr = await substringFunction([stringPtr, sliceStart, sliceLength]);
    const substring = stringStorage.loadString(substringPtr);

    expect(substring).toBe(message.slice(sliceStart, sliceLength));
});

test('concat returns the right string', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const firstMsg = 'Hello ';
    const secondMsg = 'world!';
    const firstStrPointer = stringStorage.storeUnlabeledString(firstMsg);
    const secondStrPointer = stringStorage.storeUnlabeledString(secondMsg);
    const concatFunction = runtime.getFunction('concat');
    const newStrPointer = await concatFunction([firstStrPointer, secondStrPointer]);
    const newStr = stringStorage.loadString(newStrPointer);

    expect(newStr).toBe(firstMsg + secondMsg);
});

test('not returns 0 or 1 for corresponding inputs', async () => {
    const { runtime } = runtimeDependenciesFactory();

    const notFunction = runtime.getFunction('not');
    const notTruthy = await notFunction([42]);
    const notFalsy = await notFunction([0]);

    expect(notTruthy).toBe(0);
    expect(notFalsy).toBe(1);
});

test('allocArray correctly stores an array', async () => {
    const { memMap, runtime } = runtimeDependenciesFactory();

    const size = 5;
    const init = 42;
    const allocArrayFunction = runtime.getFunction('+alloc_array');
    const arrayPointer = await allocArrayFunction([size, init]);

    for (let i = 0; i < size; i++) {
        const element = memMap.get(arrayPointer + i * WORD_SZ);
        expect(element).toBe(init);
    }
});

test('allocRecord correctly stores a record', async () => {
    const { memMap, runtime } = runtimeDependenciesFactory();

    const values = [10, 20, 30, 40, 50];
    const size = values.length;

    const allocRecordFunction = runtime.getFunction('+alloc_record');
    const arrayPointer = await allocRecordFunction([size, ...values]);

    for (let i = 0; i < size; i++) {
        const expectedElement = values[i];
        const memElement = memMap.get(arrayPointer + i * WORD_SZ);
        expect(memElement).toBe(expectedElement);
    }
});

test('+str_equals returns 0 or 1 for corresponding inputs', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const strEqualsFunction = runtime.getFunction('+str_equals');

    const leftStr1 = 'abc';
    const rightStr1 = 'abc';
    const leftStr1Pointer = stringStorage.storeUnlabeledString(leftStr1);
    const rightStr1Pointer = stringStorage.storeUnlabeledString(rightStr1);
    const firstComparison = await strEqualsFunction([leftStr1Pointer, rightStr1Pointer]);

    expect(firstComparison).toBe(1);

    const leftStr2 = 'abc';
    const rightStr2 = 'cba';
    const leftStr2Pointer = stringStorage.storeUnlabeledString(leftStr2);
    const rightStr2Pointer = stringStorage.storeUnlabeledString(rightStr2);
    const secondComparison = await strEqualsFunction([leftStr2Pointer, rightStr2Pointer]);

    expect(secondComparison).toBe(0);
});

test('+str_not_equals returns 0 or 1 for corresponding inputs', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const strEqualsFunction = runtime.getFunction('+str_not_equals');

    const leftStr1 = 'abc';
    const rightStr1 = 'abc';
    const leftStr1Pointer = stringStorage.storeUnlabeledString(leftStr1);
    const rightStr1Pointer = stringStorage.storeUnlabeledString(rightStr1);
    const firstComparison = await strEqualsFunction([leftStr1Pointer, rightStr1Pointer]);

    expect(firstComparison).toBe(0);

    const leftStr2 = 'abc';
    const rightStr2 = 'cba';
    const leftStr2Pointer = stringStorage.storeUnlabeledString(leftStr2);
    const rightStr2Pointer = stringStorage.storeUnlabeledString(rightStr2);
    const secondComparison = await strEqualsFunction([leftStr2Pointer, rightStr2Pointer]);

    expect(secondComparison).toBe(1);
});

test('+str_less returns 0 or 1 for corresponding inputs', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const strLessFunction = runtime.getFunction('+str_less');

    const leftStrEdge = 'abc';
    const rightStrEdge = 'abc';
    const leftStrEdgePointer = stringStorage.storeUnlabeledString(leftStrEdge);
    const rightStrEdgePointer = stringStorage.storeUnlabeledString(rightStrEdge);
    const edgeComparison = await strLessFunction([leftStrEdgePointer, rightStrEdgePointer]);

    expect(edgeComparison).toBe(0);

    const leftStr1 = 'abc';
    const rightStr1 = 'cba';
    const leftStr1Pointer = stringStorage.storeUnlabeledString(leftStr1);
    const rightStr1Pointer = stringStorage.storeUnlabeledString(rightStr1);
    const firstComparison = await strLessFunction([leftStr1Pointer, rightStr1Pointer]);

    expect(firstComparison).toBe(1);

    const leftStr2 = 'cba';
    const rightStr2 = 'abc';
    const leftStr2Pointer = stringStorage.storeUnlabeledString(leftStr2);
    const rightStr2Pointer = stringStorage.storeUnlabeledString(rightStr2);
    const secondComparison = await strLessFunction([leftStr2Pointer, rightStr2Pointer]);

    expect(secondComparison).toBe(0);
});

test('+str_less_or_equals returns 0 or 1 for corresponding inputs', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const strLessOrEqualsFunction = runtime.getFunction('+str_less_or_equals');

    const leftStrEdge = 'abc';
    const rightStrEdge = 'abc';
    const leftStrEdgePointer = stringStorage.storeUnlabeledString(leftStrEdge);
    const rightStrEdgePointer = stringStorage.storeUnlabeledString(rightStrEdge);
    const edgeComparison = await strLessOrEqualsFunction([leftStrEdgePointer, rightStrEdgePointer]);

    expect(edgeComparison).toBe(1);

    const leftStr1 = 'abc';
    const rightStr1 = 'cba';
    const leftStr1Pointer = stringStorage.storeUnlabeledString(leftStr1);
    const rightStr1Pointer = stringStorage.storeUnlabeledString(rightStr1);
    const firstComparison = await strLessOrEqualsFunction([leftStr1Pointer, rightStr1Pointer]);

    expect(firstComparison).toBe(1);

    const leftStr2 = 'cba';
    const rightStr2 = 'abc';
    const leftStr2Pointer = stringStorage.storeUnlabeledString(leftStr2);
    const rightStr2Pointer = stringStorage.storeUnlabeledString(rightStr2);
    const secondComparison = await strLessOrEqualsFunction([leftStr2Pointer, rightStr2Pointer]);

    expect(secondComparison).toBe(0);
});

test('+str_greater returns 0 or 1 for corresponding inputs', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const strGreaterFunction = runtime.getFunction('+str_greater');

    const leftStrEdge = 'abc';
    const rightStrEdge = 'abc';
    const leftStrEdgePointer = stringStorage.storeUnlabeledString(leftStrEdge);
    const rightStrEdgePointer = stringStorage.storeUnlabeledString(rightStrEdge);
    const edgeComparison = await strGreaterFunction([leftStrEdgePointer, rightStrEdgePointer]);

    expect(edgeComparison).toBe(0);

    const leftStr1 = 'abc';
    const rightStr1 = 'cba';
    const leftStr1Pointer = stringStorage.storeUnlabeledString(leftStr1);
    const rightStr1Pointer = stringStorage.storeUnlabeledString(rightStr1);
    const firstComparison = await strGreaterFunction([leftStr1Pointer, rightStr1Pointer]);

    expect(firstComparison).toBe(0);

    const leftStr2 = 'cba';
    const rightStr2 = 'abc';
    const leftStr2Pointer = stringStorage.storeUnlabeledString(leftStr2);
    const rightStr2Pointer = stringStorage.storeUnlabeledString(rightStr2);
    const secondComparison = await strGreaterFunction([leftStr2Pointer, rightStr2Pointer]);

    expect(secondComparison).toBe(1);
});

test('+str_greater_or_equals returns 0 or 1 for corresponding inputs', async () => {
    const { stringStorage, runtime } = runtimeDependenciesFactory();

    const strGreaterOrEqualsFunction = runtime.getFunction('+str_greater_or_equals');

    const leftStrEdge = 'abc';
    const rightStrEdge = 'abc';
    const leftStrEdgePointer = stringStorage.storeUnlabeledString(leftStrEdge);
    const rightStrEdgePointer = stringStorage.storeUnlabeledString(rightStrEdge);
    const edgeComparison = await strGreaterOrEqualsFunction([
        leftStrEdgePointer,
        rightStrEdgePointer,
    ]);

    expect(edgeComparison).toBe(1);

    const leftStr1 = 'abc';
    const rightStr1 = 'cba';
    const leftStr1Pointer = stringStorage.storeUnlabeledString(leftStr1);
    const rightStr1Pointer = stringStorage.storeUnlabeledString(rightStr1);
    const firstComparison = await strGreaterOrEqualsFunction([leftStr1Pointer, rightStr1Pointer]);

    expect(firstComparison).toBe(0);

    const leftStr2 = 'cba';
    const rightStr2 = 'abc';
    const leftStr2Pointer = stringStorage.storeUnlabeledString(leftStr2);
    const rightStr2Pointer = stringStorage.storeUnlabeledString(rightStr2);
    const secondComparison = await strGreaterOrEqualsFunction([leftStr2Pointer, rightStr2Pointer]);

    expect(secondComparison).toBe(1);
});
