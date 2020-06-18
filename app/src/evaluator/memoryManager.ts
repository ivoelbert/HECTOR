import { assertCondition } from '../interpreter/utils/utils';
import { WORD_SZ } from '../interpreter/frame';

function alignToNextWord(value: number): number {
    return ((value + WORD_SZ - 1) / WORD_SZ) * WORD_SZ;
}

export const MEMORY_PAGES = 256;
export const MEMORY_LENGTH = MEMORY_PAGES * 64 * 1024; // 16mb

export const HEAP_START = alignToNextWord(Math.floor(MEMORY_LENGTH / 3));
export const HEAP_END = HEAP_START * 2;

export const ASYNCIFY_DATA_START = HEAP_END;
export const ASYNCIFY_DATA_END = MEMORY_LENGTH;

export const i32_SIZE = 4;
export const BYTE_SIZE = 1;

export class MemoryManager {
    private allocatedSizes: Map<number, number>;
    private nextFreeIndex: number;

    constructor(private memory: Uint8Array) {
        this.nextFreeIndex = HEAP_START;
        this.allocatedSizes = new Map();
    }

    alloc = (bytes: number): number => {
        const pointer = this.nextFreeIndex;
        this.nextFreeIndex += bytes;
        assertCondition(this.nextFreeIndex < HEAP_END, 'Out of memory!');
        this.allocatedSizes.set(pointer, bytes);
        return pointer;
    };

    checkArrayIndex = (pointer: number, index: number): void => {
        const byteCount = this.allocatedSizes.get(pointer);
        if (byteCount === undefined) {
            throw new Error('Not a valid array');
        }
        if (index >= byteCount * i32_SIZE || index < 0) {
            throw new Error('Index out of bounds');
        }

        return;
    };

    i32Store = (dir: number, value: number): void => {
        i32AssertRange(dir);

        this.memory[dir] = value & 255;
        this.memory[dir + 1] = (value & (255 << 8)) >> 8;
        this.memory[dir + 2] = (value & (255 << 16)) >> 16;
        this.memory[dir + 3] = (value & (255 << 24)) >> 24;
    };

    i32Get = (dir: number): number => {
        i32AssertRange(dir);

        let value = this.memory[dir];
        value += this.memory[dir + 1] << 8;
        value += this.memory[dir + 2] << 16;
        value += this.memory[dir + 3] << 24;

        return value;
    };

    byteStore = (dir: number, value: number): void => {
        byteAssertRange(dir);

        this.memory[dir] = value;
    };

    byteGet = (dir: number): number => {
        byteAssertRange(dir);

        return this.memory[dir];
    };

    i32DebugSlice = (start: number, count: number): void => {
        const values = [];
        for (let i = 0; i < count; i++) {
            const dir = start + i * i32_SIZE;
            values.push(this.i32Get(dir));
        }

        console.log(values);
    };

    byteDebugSlice = (start: number, count: number): void => {
        const values = [];
        for (let i = 0; i < count; i++) {
            const dir = start + i * BYTE_SIZE;
            values.push(this.byteGet(dir));
        }

        console.log(values);
    };
}

const i32AssertRange = (dir: number): void => {
    assertCondition(dir >= 0 && dir < HEAP_END - 4, `Index ${dir} out of range`);
};

const byteAssertRange = (dir: number): void => {
    assertCondition(dir >= 0 && dir < HEAP_END - 1, `Index ${dir} out of range`);
};
