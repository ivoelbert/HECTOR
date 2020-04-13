export class NotImplementedError extends Error {
    constructor() {
        super('Not implemented!');
    }
}

export class UnexpectedUndefinedError extends Error {
    constructor(msg?: string) {
        super(msg ?? 'Unexpected undefined value!');
    }
}

export class UnreachableError extends Error {
    constructor(msg?: string) {
        super(msg ?? 'Unreachable branch');
    }
}

export const assertExists = <T>(value: T | undefined, msg?: string): T => {
    if (value === undefined) {
        throw new UnexpectedUndefinedError(msg);
    } else {
        return value;
    }
};

interface IncludeMapEntry<K, V> {
    key: K;
    value: V;
}

export class IncludeMap<V> {
    entries: IncludeMapEntry<string, V>[];

    constructor() {
        this.entries = [];
    }

    set = (key: string, value: V): IncludeMap<V> => {
        const foundIndex = this.entries.findIndex((entry): boolean =>
            entry.key.includes(key)
        );

        if (foundIndex >= 0) {
            this.entries[foundIndex] = { key, value };
        } else {
            this.entries.push({ key, value });
        }

        return this;
    };

    get = (key: string): V | undefined => {
        const maybeEntry = this.entries.find((entry): boolean =>
            entry.key.includes(key)
        );

        if (maybeEntry === undefined) {
            return undefined;
        } else {
            return maybeEntry.value;
        }
    };
}
