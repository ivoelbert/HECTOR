import { MemMap } from './utils/memMap';
import { assertExists } from './utils/utils';
import { StringStorage } from './utils/stringStorage';
import { CustomConsole } from '../utils/console';
import { WORD_SZ } from '../utils/utils';
import { OutOfBoundsException, NilPointerException } from '../utils/runtimeUtils';

interface RuntimeFunctionsByName {
    print: RuntimeFunction | AsyncRuntimeFunction;
    flush: RuntimeFunction | AsyncRuntimeFunction;
    getchar: RuntimeFunction | AsyncRuntimeFunction;
    getstring: RuntimeFunction | AsyncRuntimeFunction;
    ord: RuntimeFunction | AsyncRuntimeFunction;
    chr: RuntimeFunction | AsyncRuntimeFunction;
    size: RuntimeFunction | AsyncRuntimeFunction;
    substring: RuntimeFunction | AsyncRuntimeFunction;
    concat: RuntimeFunction | AsyncRuntimeFunction;
    not: RuntimeFunction | AsyncRuntimeFunction;
    '+alloc_array': RuntimeFunction | AsyncRuntimeFunction;
    '+alloc_record': RuntimeFunction | AsyncRuntimeFunction;
    '+str_equals': RuntimeFunction | AsyncRuntimeFunction;
    '+str_not_equals': RuntimeFunction | AsyncRuntimeFunction;
    '+str_less': RuntimeFunction | AsyncRuntimeFunction;
    '+str_less_or_equals': RuntimeFunction | AsyncRuntimeFunction;
    '+str_greater': RuntimeFunction | AsyncRuntimeFunction;
    '+str_greater_or_equals': RuntimeFunction | AsyncRuntimeFunction;
    '+check_index_array': RuntimeFunction | AsyncRuntimeFunction;
    '+check_nil': RuntimeFunction | AsyncRuntimeFunction;
}

export type RuntimeFunctionNames = keyof RuntimeFunctionsByName;

type RuntimeFunction = (args: number[]) => number;
type AsyncRuntimeFunction = (args: number[]) => Promise<number>;

export class Runtime {
    private nameMap: RuntimeFunctionsByName;
    private arraySizes: Map<number, number>;

    constructor(
        private memMap: MemMap,
        private stringStorage: StringStorage,
        private console: CustomConsole
    ) {
        this.arraySizes = new Map();
        this.nameMap = {
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
            '+alloc_array': this.allocArray,
            '+alloc_record': this.allocRecord,
            '+str_equals': this.strEquals,
            '+str_not_equals': this.strNotEquals,
            '+str_less': this.strLess,
            '+str_less_or_equals': this.strLessOrEquals,
            '+str_greater': this.strGreater,
            '+str_greater_or_equals': this.strGreaterOrEquals,
            '+check_index_array': this.checkIndexArray,
            '+check_nil': this.checkNil,
        };
    }

    getFunction = (name: RuntimeFunctionNames): RuntimeFunction | AsyncRuntimeFunction => {
        return this.nameMap[name];
    };

    maybeGetFunction = (name: string): RuntimeFunction | AsyncRuntimeFunction | undefined => {
        return this.nameMap[name as RuntimeFunctionNames];
    };

    /**
     *  Public functions
     */

    private print: RuntimeFunction = (args) => {
        const [strPointer] = args;
        assertExists(strPointer);

        const str = this.stringStorage.loadString(strPointer);
        this.console.print(str);

        return 0;
    };

    private flush: RuntimeFunction = (args) => {
        return 0;
    };

    private getchar: AsyncRuntimeFunction = async (args) => {
        const str = await this.console.readChar();
        return this.stringStorage.storeUnlabeledString(str[0]);
    };

    private getstring: AsyncRuntimeFunction = async (args) => {
        const str = await this.console.read();
        return this.stringStorage.storeUnlabeledString(str);
    };

    private ord: RuntimeFunction = (args) => {
        const [strPointer] = args;
        assertExists(strPointer);

        const str = this.stringStorage.loadString(strPointer);

        return str.charCodeAt(0);
    };

    private chr: RuntimeFunction = (args) => {
        const [charCode] = args;
        assertExists(charCode);

        const str = String.fromCharCode(charCode);
        return this.stringStorage.storeUnlabeledString(str);
    };

    private size: RuntimeFunction = (args) => {
        const [strPointer] = args;
        assertExists(strPointer);

        const str = this.stringStorage.loadString(strPointer);
        return str.length;
    };

    private substring: RuntimeFunction = (args) => {
        const [strPointer, start, count] = args;
        assertExists(strPointer);

        const str = this.stringStorage.loadString(strPointer);
        const slicedStr = str.slice(start, count);

        return this.stringStorage.storeUnlabeledString(slicedStr);
    };

    private concat: RuntimeFunction = (args) => {
        const [leftStrPointer, rightStrPointer] = args;
        assertExists(leftStrPointer);
        assertExists(rightStrPointer);

        const leftStr = this.stringStorage.loadString(leftStrPointer);
        const rightStr = this.stringStorage.loadString(rightStrPointer);
        const newStr = leftStr + rightStr;

        return this.stringStorage.storeUnlabeledString(newStr);
    };

    private not: RuntimeFunction = (args) => {
        const [value] = args;
        assertExists(value);

        return Number(!value);
    };

    /**
     *  Internal functions
     */

    private allocArray: RuntimeFunction = (args) => {
        const [size, init] = args;
        assertExists(size);
        assertExists(init);

        const arrayLocation = this.memMap.alloc(size);
        this.arraySizes.set(arrayLocation, size);

        for (let i = 0; i < size; i++) {
            const itemLocation = arrayLocation + i * WORD_SZ;

            this.memMap.set(itemLocation, init);
        }

        return arrayLocation;
    };

    private checkIndexArray: RuntimeFunction = (args) => {
        const [pointer, index] = args;
        assertExists(pointer);
        assertExists(index);

        const size = this.arraySizes.get(pointer);

        if (index < 0 || index >= assertExists(size)) {
            throw new OutOfBoundsException(index, pointer);
        }

        return 0;
    };

    private checkNil: RuntimeFunction = (args) => {
        const [record] = args;
        assertExists(record);

        if (record === 0) {
            throw new NilPointerException();
        }

        return 0;
    };

    private allocRecord: RuntimeFunction = (args) => {
        const [size, ...values] = args;
        assertExists(size);

        const recordLocation = this.memMap.alloc(size);

        for (let i = 0; i < size; i++) {
            const itemLocation = recordLocation + i * WORD_SZ;
            const item = assertExists(values[i]);

            this.memMap.set(itemLocation, item);
        }

        return recordLocation;
    };

    private strEquals: RuntimeFunction = (args) => {
        const comparison = this.strCompare(args);
        return Number(comparison === 0);
    };

    private strNotEquals: RuntimeFunction = (args) => {
        const comparison = this.strCompare(args);
        return Number(comparison !== 0);
    };

    private strLess: RuntimeFunction = (args) => {
        const comparison = this.strCompare(args);
        return Number(comparison < 0);
    };

    private strLessOrEquals: RuntimeFunction = (args) => {
        const comparison = this.strCompare(args);
        return Number(comparison <= 0);
    };

    private strGreater: RuntimeFunction = (args) => {
        const comparison = this.strCompare(args);
        return Number(comparison > 0);
    };

    private strGreaterOrEquals: RuntimeFunction = (args) => {
        const comparison = this.strCompare(args);
        return Number(comparison >= 0);
    };

    // extracts the strings and returns their localCompare
    private strCompare: RuntimeFunction = (args) => {
        const [leftStrPointer, rightStrPointer] = args;

        const leftStr = this.stringStorage.loadString(assertExists(leftStrPointer));
        const rightStr = this.stringStorage.loadString(assertExists(rightStrPointer));

        return leftStr.localeCompare(rightStr);
    };
}
