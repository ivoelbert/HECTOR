import { CustomConsole } from '../utils/console';
import { MemoryManager, i32_SIZE, HEAP_START } from './memoryManager';

type TigerMain = () => number;

interface InstanceExports {
    main: TigerMain;
    memory: Uint8Array;
}

export class Runtime {
    private wasmInstance: WebAssembly.Instance;
    private memoryManager: MemoryManager;

    constructor(module: WebAssembly.Module, private customConsole: CustomConsole) {
        this.wasmInstance = new WebAssembly.Instance(module, {
            externals: {
                print: this.print,
                flush: this.flush,
                getchar: this.getchar,
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

        this.memoryManager = new MemoryManager(this.exports.memory);
    }

    run = (): number => {
        const execution = this.exports.main();
        return execution;
    };

    private get exports(): InstanceExports {
        return {
            main: this.wasmInstance.exports.tigermain_wrapper as TigerMain,
            memory: new Uint8Array((this.wasmInstance.exports.memory as WebAssembly.Memory).buffer),
        };
    }

    private print = () => {};
    private flush = () => {};
    private getchar = () => {};
    private ord = () => {};
    private chr = () => {};
    private size = () => {};
    private substring = () => {};
    private concat = () => {};
    private not = () => {};
    private exit = () => {};
    private alloc_array = (size: number, init: number) => {
        console.log('ALLOC ARRAY!!!!!!!!!!!!!!!');
        console.log(`With size ${size}, full of ${init}`);
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
