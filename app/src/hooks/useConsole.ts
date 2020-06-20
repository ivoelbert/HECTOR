import { CustomConsole } from '../utils/console';
import { useState, useMemo } from 'react';

export interface UserConsole extends CustomConsole {
    clear: () => void;
    resolveRead: (str: string) => void;
    printLine: (str: string) => void;
}

const illegalResolve = (str: string): void => {
    throw new Error('Cannot resolve a string if not reading!');
};

export const useConsole = (): [UserConsole, string[], boolean, boolean] => {
    const [messages, setMessages] = useState<string[]>(['']);

    const [isReading, setIsReading] = useState<boolean>(false);

    const [isReadingChar, setIsReadingChar] = useState<boolean>(false);

    const hookConsole = useMemo(() => {
        class HookConsole implements UserConsole {
            resolveRead: (str: string) => void = illegalResolve;

            clear = () => {
                setMessages((_oldMessages) => ['']);
            };

            print = (str: string) => {
                setMessages((oldMessages) => {
                    const newMessages = [...oldMessages];

                    newMessages[newMessages.length - 1] = newMessages[
                        newMessages.length - 1
                    ].concat(str.replace(/\\n/g, '\n'));

                    return newMessages;
                });
            };

            printLine = (str: string) => {
                setMessages((oldMessages) => [...oldMessages, str]);
            };

            read = () => {
                return new Promise<string>((resolve) => {
                    setIsReading(true);

                    this.resolveRead = (str: string) => {
                        setMessages((oldMessages) => [...oldMessages, str, '']);
                        setIsReading(false);

                        // Send this value over to the console consumer
                        resolve(str);
                    };
                });
            };

            readChar = () => {
                return new Promise<string>((resolve) => {
                    setIsReadingChar(true);

                    this.resolveRead = (str: string) => {
                        const char = str[0];
                        setMessages((oldMessages) => [...oldMessages, char, '']);
                        setIsReadingChar(false);

                        // Send this value over to the console consumer
                        resolve(char);
                    };
                });
            };
        }

        return new HookConsole();
    }, []);

    return [hookConsole, messages, isReading, isReadingChar];
};
