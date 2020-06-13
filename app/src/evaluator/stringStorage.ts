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

    writeString = (str: string): number => {
        const pointer = this.memoryManager.alloc(str.length + 1);
        for (let i = 0; i < str.length; i++) {
            const charCode = str.charCodeAt(i);
            this.memoryManager.byteStore(pointer + i, charCode);
        }
        this.memoryManager.byteStore(pointer + str.length, 0);

        return pointer;
    };
}
