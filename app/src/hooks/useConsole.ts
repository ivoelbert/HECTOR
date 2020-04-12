import { CustomConsole } from '../interpreter/utils/console';
import { useState, useMemo } from 'react';

export interface InterpConsole extends CustomConsole {
    clear: () => void;
    resolveRead: (str: string) => void;
    printLine: (str: string) => void;
}

const illegalResolve = (str: string): void => {
    throw new Error('Cannot resolve a string if not reading!');
};

export const useConsole = (): [InterpConsole, string[], boolean] => {
    const [messages, setMessages] = useState<string[]>(['']);

    const [isReading, setIsReading] = useState<boolean>(false);

    const hookConsole = useMemo(() => {
        class HookConsole implements InterpConsole {
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
                        setMessages((oldMessages) => [
                            ...oldMessages,
                            str,
                            '',
                        ]);
                        setIsReading(false);

                        // Send this value over to the interpreter
                        resolve(str);
                    };
                });
            };
        }

        return new HookConsole();
    }, []);

    return [hookConsole, messages, isReading];
};
