import { assertCondition } from '../interpreter/utils/utils';

export const MEMORY_LENGTH = 32 * 1024 * 1024; // 32mb
export const HEAP_START = Math.floor(MEMORY_LENGTH / 2);

export const i32_SIZE = 4;
export const BYTE_SIZE = 1;

export class MemoryManager {
    private nextFreeIndex: number;

    constructor(private memory: Uint8Array) {
        this.nextFreeIndex = HEAP_START;
    }

    alloc = (bytes: number): number => {
        const pointer = this.nextFreeIndex;
        this.nextFreeIndex += bytes;
        assertCondition(this.nextFreeIndex < MEMORY_LENGTH, 'Out of memory!');
        return pointer;
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
}

const i32AssertRange = (dir: number): void => {
    assertCondition(dir >= HEAP_START && dir < MEMORY_LENGTH - 4, `Index ${dir} out of range`);
};

const byteAssertRange = (dir: number): void => {
    assertCondition(dir >= HEAP_START && dir < MEMORY_LENGTH - 1, `Index ${dir} out of range`);
};
