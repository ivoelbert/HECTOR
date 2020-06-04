import { CustomConsole } from '../src/interpreter/utils/console';

/**
 *  A mocked console that writes to an internal array
 *  and always reads the string "mock"
 */
export class MockConsole implements CustomConsole {
    private messages: string[];
    private mockReadResult: string;

    constructor() {
        this.messages = [];
        this.mockReadResult = '';
    }

    print = (msg: string): void => {
        this.messages.push(msg);
    };

    read = async (): Promise<string> => {
        return this.mockReadResult;
    };

    setReadResult = (msg: string): void => {
        this.mockReadResult = msg;
    };

    getLastMessage = (): string => {
        const lastIndex = this.messages.length - 1;
        return this.messages[lastIndex];
    };
}
