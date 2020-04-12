import { WORD_SZ } from '../frame';

export class MemMap extends Map<number, number> {
    private nextFreeMem: number = 0;

    // Optimistic alloc
    public alloc = (words: number): number => {
        const nextFree = this.nextFreeMem;
        this.nextFreeMem += words * WORD_SZ;
        return nextFree;
    };
}
