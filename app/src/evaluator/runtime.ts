import { CustomConsole } from '../utils/console';
import { MemoryManager, i32_SIZE, MEMORY_PAGES } from './memoryManager';
import { StringStorage } from './stringStorage';
import { RuntimeExit } from '../utils/runtimeExit';
import * as Asyncify from './asyncify';

type TigerMain = () => Promise<number>;

interface InstanceExports {
    main: TigerMain;
}

export class Runtime {
    private wasmInstance: WebAssembly.Instance;
    private memoryManager: MemoryManager;
    private stringStorage: StringStorage;

    constructor(module: WebAssembly.Module, private customConsole: CustomConsole) {
        const memory = new WebAssembly.Memory({ initial: MEMORY_PAGES, maximum: MEMORY_PAGES });

        this.wasmInstance = new Asyncify.Instance(module, {
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
                check_index_array: this.check_index_array,
                check_nil: this.check_nil,
                str_equals: this.str_equals,
                str_not_equals: this.str_not_equals,
                str_less: this.str_less,
                str_less_or_equals: this.str_less_or_equals,
                str_greater: this.str_greater,
                str_greater_or_equals: this.str_greater_or_equals,
            },
        });

        this.memoryManager = new MemoryManager(new Uint8Array(memory.buffer));
        this.stringStorage = new StringStorage(this.memoryManager);
    }

    run = async (): Promise<number> => {
        try {
            const execution = await this.exports.main();
            return execution;
        } catch (err) {
            if (err instanceof RuntimeExit) {
                return err.exitCode;
            } else {
                throw err;
            }
        }
    };

    private get exports(): InstanceExports {
        return {
            main: this.wasmInstance.exports.tigermain_wrapper as TigerMain,
        };
    }

    private print = (strPointer: number): void => {
        const string = this.stringStorage.readString(strPointer);
        this.customConsole.print(string);
    };
    private flush = () => {};
    private getchar = async () => {
        const string = await this.customConsole.read();
        return this.stringStorage.writeString(string);
    };
    private getstring = async () => {
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
        const pointer = this.memoryManager.alloc(size * i32_SIZE);
        for (let i = 0; i < size; i++) {
            const dir = pointer + i * i32_SIZE;
            this.memoryManager.i32Store(dir, init);
        }

        return pointer;
    };
    private alloc_record = (size: number): number => {
        const pointer = this.memoryManager.alloc(size * i32_SIZE);
        return pointer;
    };
    private check_index_array = (pointer: number, index: number): void => {
        return this.memoryManager.checkArrayIndex(pointer, index);
    };
    private check_nil = () => {};
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
