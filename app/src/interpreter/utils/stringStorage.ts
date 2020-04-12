import { MemMap } from './memMap';
import { assertExists } from './utils';
import { Label, StringFrag } from '../treeTypes';

/**
 *  StringStorage will handle strings related stuff.
 *
 *  storeString will take a StringFrag (label, string).
 *  It'll store a pointer to that string in memory (pointing
 *  to the index in a private string array) and store that
 *  mem location in the labels map.
 */
export class StringStorage {
    private strings: string[] = [];

    constructor(private memMap: MemMap, private labelMap: Map<Label, number>) {}

    storeString = (stringFrag: StringFrag): number => {
        const [label, str] = stringFrag.ConstString;

        const addr = this.storeUnlabeledString(str);

        this.labelMap.set(label, addr);

        return addr;
    };

    storeUnlabeledString = (str: string): number => {
        const addr = this.memMap.alloc(1);
        this.strings.push(str);
        const pointer = this.strings.length - 1;

        this.memMap.set(addr, pointer);

        return addr;
    };

    loadString = (addr: number): string => {
        const pointer = assertExists(this.memMap.get(addr));
        return this.strings[pointer];
    };
}
