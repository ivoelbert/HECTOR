import { MemoryManager } from './memoryManager';

export class StringStorage {
    constructor(private memoryManager: MemoryManager) {}

    readString = (strPointer: number): string => {
        let string = '';
        for (let i = 0; true; i++) {
            const encodedChar = this.memoryManager.byteGet(strPointer + i);

            if (encodedChar === 0) {
                break;
            }

            string += String.fromCharCode(encodedChar);
        }

        return string;
    };
}
