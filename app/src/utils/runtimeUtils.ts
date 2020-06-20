export class RuntimeExit {
    constructor(readonly exitCode: number) {}
}

export class OutOfBoundsException extends Error {
    constructor(readonly index: number, readonly pointer: number) {
        super(`Cannot access pointer ${pointer} at index ${index}`);
    }
}

export class NilPointerException extends Error {
    constructor() {
        super('Tried accessing nil pointer field');
    }
}
