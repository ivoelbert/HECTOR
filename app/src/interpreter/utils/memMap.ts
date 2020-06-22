import { WORD_SZ } from '../../utils/utils';
import { HEAP_START } from '../../evaluator/memoryManager';

export class MemMap extends Map<number, number> {
    private nextFreeMem: number = HEAP_START;

    // Optimistic alloc
    public alloc = (words: number): number => {
        const nextFree = this.nextFreeMem;
        this.nextFreeMem += words * WORD_SZ;
        return nextFree;
    };
}
