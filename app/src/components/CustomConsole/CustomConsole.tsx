import React, { useState, useRef, useEffect } from 'react';
import { UserConsole } from '../../hooks/useConsole';

import './CustomConsole.scss';

const prompt = '>';

interface CustomConsoleProps {
    customConsole: UserConsole;
    messages: string[];
    isReading: boolean;
    isReadingChar: boolean;
}

export const CustomConsole: React.FC<CustomConsoleProps> = (props) => {
    const { messages, isReading, isReadingChar, customConsole } = props;

    const consoleRef = useRef<HTMLDivElement | null>(null);
    useEffect(() => {
        const element = consoleRef.current;
        if (element !== null) {
            element.scrollTop = element.scrollHeight;
        }
    });

    return (
        <div className="custom-console" ref={consoleRef}>
            {messages.map((msg: string, index: number) => {
                return <MessageWithPrompt key={index}>{msg}</MessageWithPrompt>;
            })}
            {isReading && <ConsoleInput customConsole={customConsole} />}
            {isReadingChar && <ConsoleCharInput customConsole={customConsole} />}
        </div>
    );
};

interface MessageWithPromptProps {
    children: string;
}

const MessageWithPrompt: React.FC<MessageWithPromptProps> = (props) => {
    const message = props.children;

    return (
        <div className="message-with-prompt">
            <span className="prompt">{prompt}</span>
            <pre className="message">{message}</pre>
        </div>
    );
};

interface ConsoleInputProps {
    customConsole: UserConsole;
}

const ConsoleInput: React.FC<ConsoleInputProps> = (props) => {
    const [message, setMessage] = useState('');

    const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const { value } = event.target;

        setMessage(value);
    };

    const onKeyPress = (event: React.KeyboardEvent<HTMLInputElement>) => {
        const { key } = event;

        if (key === 'Enter') {
            props.customConsole.resolveRead(message);
        }
    };

    return (
        <div className="message-with-prompt">
            <span className="prompt">{prompt}</span>
            <input
                className="input"
                autoFocus={true}
                onChange={onChange}
                value={message}
                onKeyPress={onKeyPress}
                spellCheck={false}
            />
        </div>
    );
};

const ConsoleCharInput: React.FC<ConsoleInputProps> = (props) => {
    const [message, setMessage] = useState<string>('');
    const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const { value } = event.target;
        setMessage(value);

        if (value.length >= 1) {
            props.customConsole.resolveRead(value[0]);
        }
    };

    return (
        <div className="message-with-prompt">
            <span className="prompt">{prompt}</span>
            <input
                className="input"
                autoFocus={true}
                onChange={onChange}
                value={message}
                spellCheck={false}
            />
        </div>
    );
};
