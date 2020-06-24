import { CustomConsole } from '../utils/console';
import {
    MemoryManager,
    MEMORY_PAGES,
    ASYNCIFY_DATA_START,
    ASYNCIFY_DATA_END,
} from './memoryManager';
import { StringStorage } from './stringStorage';
import { RuntimeExit, NilPointerException } from '../utils/runtimeUtils';
import * as Asyncify from './asyncify';
import { WORD_SZ } from '../utils/utils';

type TigerMain = () => Promise<number>;

export class Runtime {
    private memoryManager: MemoryManager;
    private stringStorage: StringStorage;
    private instanceImports: Record<string, Record<string, WebAssembly.ImportValue>>;

    constructor(private binary: Uint8Array, private customConsole: CustomConsole) {
        const memory = new WebAssembly.Memory({ initial: MEMORY_PAGES, maximum: MEMORY_PAGES });

        this.memoryManager = new MemoryManager(new Uint8Array(memory.buffer));
        this.stringStorage = new StringStorage(this.memoryManager);
        this.instanceImports = {
            mem: {
                memory,
            },
            externals: {
                print: this.print,
                flush: this.flush,
                getchar: this.getchar,
                getstring: this.getstring,
                ord: this.ord,
                chr: this.chr,
                size: this.size,
                substring: this.substring,
                concat: this.concat,
                not: this.not,
                exit: this.exit,
                alloc_array: this.alloc_array,
                alloc_record: this.alloc_record,
                debug_words: this.debug_words,
                debug_bytes: this.debug_bytes,
                check_index_array: this.check_index_array,
                check_nil: this.check_nil,
                str_equals: this.str_equals,
                str_not_equals: this.str_not_equals,
                str_less: this.str_less,
                str_less_or_equals: this.str_less_or_equals,
                str_greater: this.str_greater,
                str_greater_or_equals: this.str_greater_or_equals,
            },
        };
    }

    run = async (): Promise<number> => {
        try {
            const wasmInstance = await Asyncify.instantiate(
                this.binary,
                ASYNCIFY_DATA_START,
                ASYNCIFY_DATA_END,
                this.instanceImports
            );

            const main = wasmInstance.instance.exports.tigermain_wrapper as TigerMain;
            const execution = await main();

            return execution;
        } catch (err) {
            if (err instanceof RuntimeExit) {
                return err.exitCode;
            } else {
                throw err;
            }
        }
    };

    private print = (strPointer: number): number => {
        const string = this.stringStorage.readString(strPointer);
        this.customConsole.print(string);
        return 0;
    };
    private flush = (): number => {
        return 0;
    };
    private getchar = async (): Promise<number> => {
        const string = await this.customConsole.readChar();
        return this.stringStorage.writeString(string);
    };
    private getstring = async (): Promise<number> => {
        const string = await this.customConsole.read();
        return this.stringStorage.writeString(string);
    };
    private ord = (strPointer: number): number => {
        const string = this.stringStorage.readString(strPointer);
        return string.charCodeAt(0);
    };
    private chr = (charCode: number): number => {
        const string = String.fromCharCode(charCode);
        return this.stringStorage.writeString(string);
    };
    private size = (strPointer: number): number => {
        const string = this.stringStorage.readString(strPointer);
        return string.length;
    };
    private substring = (strPointer: number, start: number, end: number): number => {
        const string = this.stringStorage.readString(strPointer);
        const slicedString = string.slice(start, end);
        const newStrPointer = this.stringStorage.writeString(slicedString);
        return newStrPointer;
    };
    private concat = (str1Pointer: number, str2Pointer: number): number => {
        const str1 = this.stringStorage.readString(str1Pointer);
        const str2 = this.stringStorage.readString(str2Pointer);

        return this.stringStorage.writeString(str1 + str2);
    };
    private not = (condition: number): number => {
        return Number(!condition);
    };
    private exit = (exitCode: number) => {
        throw new RuntimeExit(exitCode);
    };
    private alloc_array = (size: number, init: number): number => {
        const pointer = this.memoryManager.alloc(size * WORD_SZ);
        for (let i = 0; i < size; i++) {
            const dir = pointer + i * WORD_SZ;
            this.memoryManager.wordStore(dir, init);
        }

        return pointer;
    };
    private alloc_record = (size: number): number => {
        const pointer = this.memoryManager.alloc(size * WORD_SZ);
        console.log(`Alloc'd record in ${pointer}`);
        return pointer;
    };
    private debug_words = (pointer: number, words: number) => {
        console.log(`Will print ${words} words from pointer ${pointer}`);
        this.memoryManager.wordDebugSlice(pointer, words);
        return 0;
    };
    private debug_bytes = (pointer: number, bytes: number) => {
        console.log(`Will print ${bytes} bytes from pointer ${pointer}`);
        this.memoryManager.byteDebugSlice(pointer, bytes);
        return 0;
    };
    private check_index_array = (pointer: number, index: number): number => {
        this.memoryManager.checkArrayIndex(pointer, index);
        return 0;
    };
    private check_nil = (record: number): number => {
        if (record === 0) {
            throw new NilPointerException();
        }
        return 0;
    };
    private str_equals = (leftStrPointer: number, rightStrPointer: number): number => {
        const comparison = this.strCompare(leftStrPointer, rightStrPointer);

        return Number(comparison === 0);
    };
    private str_not_equals = (leftStrPointer: number, rightStrPointer: number): number => {
        const comparison = this.strCompare(leftStrPointer, rightStrPointer);

        return Number(comparison !== 0);
    };
    private str_less = (leftStrPointer: number, rightStrPointer: number): number => {
        const comparison = this.strCompare(leftStrPointer, rightStrPointer);

        return Number(comparison < 0);
    };
    private str_less_or_equals = (leftStrPointer: number, rightStrPointer: number): number => {
        const comparison = this.strCompare(leftStrPointer, rightStrPointer);

        return Number(comparison <= 0);
    };
    private str_greater = (leftStrPointer: number, rightStrPointer: number): number => {
        const comparison = this.strCompare(leftStrPointer, rightStrPointer);

        return Number(comparison > 0);
    };
    private str_greater_or_equals = (leftStrPointer: number, rightStrPointer: number): number => {
        const comparison = this.strCompare(leftStrPointer, rightStrPointer);

        return Number(comparison >= 0);
    };

    private strCompare = (leftStrPointer: number, rightStrPointer: number): number => {
        const leftStr = this.stringStorage.readString(leftStrPointer);
        const rightStr = this.stringStorage.readString(rightStrPointer);

        return leftStr.localeCompare(rightStr);
    };
}
