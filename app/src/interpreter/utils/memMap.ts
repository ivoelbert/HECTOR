import { WORD_SZ } from '../../utils/utils';
import { HEAP_START } from '../../evaluator/memoryManager';
import { assertCondition } from './utils';

export class MemMap extends Map<number, number> {
    private nextFreeMem: number = HEAP_START;

    // Optimistic alloc
    public alloc = (bytes: number): number => {
        assertCondition(bytes > 0, 'Cannot alloc 0 words');
        const nextFree = this.nextFreeMem;
        this.nextFreeMem += bytes * WORD_SZ;
        return nextFree;
    };
}
