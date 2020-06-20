export interface CustomConsole {
    print(msg: string): void;
    read(): Promise<string>;
    readChar(): Promise<string>;
}
