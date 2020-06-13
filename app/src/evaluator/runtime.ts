import { CustomConsole } from '../utils/console';
import { MemoryManager, i32_SIZE, MEMORY_PAGES } from './memoryManager';
import { StringStorage } from './stringStorage';

type TigerMain = () => number;

interface InstanceExports {
    main: TigerMain;
}

export class Runtime {
    private wasmInstance: WebAssembly.Instance;
    private memoryManager: MemoryManager;
    private stringStorage: StringStorage;

    constructor(module: WebAssembly.Module, private customConsole: CustomConsole) {
        const memory = new WebAssembly.Memory({ initial: MEMORY_PAGES, maximum: MEMORY_PAGES });

        this.wasmInstance = new WebAssembly.Instance(module, {
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

    run = (): number => {
        const execution = this.exports.main();
        return execution;
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
    private getchar = () => {};
    private getstring = () => {};
    private ord = () => {};
    private chr = () => {};
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
    private not = () => {};
    private exit = () => {};
    private alloc_array = (size: number, init: number): number => {
        const pointer = this.memoryManager.alloc(size * i32_SIZE);
        for (let i = 0; i < size; i++) {
            const dir = pointer + i * i32_SIZE;
            this.memoryManager.i32Store(dir, init);
        }

        return pointer;
    };
    private alloc_record = () => {};
    private check_index_array = () => {};
    private check_nil = () => {};
    private str_equals = () => {};
    private str_not_equals = () => {};
    private str_less = () => {};
    private str_less_or_equals = () => {};
    private str_greater = () => {};
    private str_greater_or_equals = () => {};
}
