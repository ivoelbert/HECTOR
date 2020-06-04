import React, { useState, useRef, useEffect } from 'react';
import { InterpConsole } from '../../hooks/useConsole';

import './CustomConsole.scss';

const prompt = '>';

interface CustomConsoleProps {
    customConsole: InterpConsole;
    messages: string[];
    isReading: boolean;
}

export const CustomConsole: React.FC<CustomConsoleProps> = (props) => {
    const { messages, isReading, customConsole } = props;

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
    customConsole: InterpConsole;
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
