import { assertCondition } from '../interpreter/utils/utils';

const MEMORY_LENGTH = 4 * 8 * 1024 * 1024; // space for 134217728 i32s
const HEAP_START = Math.floor(MEMORY_LENGTH / 2);
const STACK_START = 0;

export class MemoryManager {
    private nextFreeIndex: number;

    constructor(private memory: Uint8Array) {
        this.nextFreeIndex = HEAP_START;
        console.log(memory.length);
    }

    alloc = (bytes: number): number => {
        const pointer = this.nextFreeIndex;
        this.nextFreeIndex += bytes;
        if (this.nextFreeIndex > MEMORY_LENGTH) {
            throw new Error('OUT OF MEMORY!');
        }
        return pointer;
    };

    i32Store = (dir: number, value: number): void => {
        assertCondition(dir > HEAP_START && dir < MEMORY_LENGTH - 4, `Index ${dir} out of range`);

        this.memory[dir] = value & 255;
        this.memory[dir + 1] = (value & (255 << 8)) >> 8;
        this.memory[dir + 1] = (value & (255 << 16)) >> 16;
        this.memory[dir + 1] = (value & (255 << 24)) >> 24;
    };
}
