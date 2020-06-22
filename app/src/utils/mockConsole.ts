import { CustomConsole } from './console';

/**
 *  A mocked console that writes to an internal array
 *  and always reads the string "mock"
 */
export class MockConsole implements CustomConsole {
    private printedText: string;
    private mockReadResult: string;

    constructor() {
        this.printedText = '';
        this.mockReadResult = '';
    }

    print = (msg: string): void => {
        this.printedText += msg;
    };

    read = async (): Promise<string> => {
        return this.mockReadResult;
    };

    readChar = async (): Promise<string> => {
        return this.mockReadResult[0];
    };

    setReadResult = (msg: string): void => {
        this.mockReadResult = msg;
    };

    getPrintedText = (): string => {
        return this.printedText;
    };
}
