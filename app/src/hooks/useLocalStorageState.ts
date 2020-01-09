import { useState } from 'react';

export const useLocalStorageState = <T>(
    key: string,
    initialValue: T
): [T, React.Dispatch<React.SetStateAction<T>>] => {
    // Pass initial state function to useState so logic is only executed once
    const [storedValue, setStoredValue] = useState<T>(() => {
        try {
            const item: string | null = window.localStorage.getItem(key);
            // Parse stored json or if none return initialValue
            return item ? JSON.parse(item) : initialValue;
        } catch (error) {
            // If error also return initialValue
            console.log(error);
            return initialValue;
        }
    });

    // Return a wrapped version of useState's setter function that persists the new value to localStorage.
    const setValue: React.Dispatch<React.SetStateAction<T>> = (value): void => {
        try {
            // Allow value to be a function so we have same API as useState
            const valueToStore: T =
                value instanceof Function ? value(storedValue) : value;

            setStoredValue(valueToStore);

            window.localStorage.setItem(key, JSON.stringify(valueToStore));
        } catch (error) {
            // We'll suppose you have localstorage, if not well... Just log for now.
            console.warn(error);
        }
    };

    return [storedValue, setValue];
};
