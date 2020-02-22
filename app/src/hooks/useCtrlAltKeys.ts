import { useEffect } from 'react';

export const useCtrlAltKeys = (compileKeyCodes: number[], handler: () => void): void => {
    useEffect(() => {
        const keyHandler = (e: KeyboardEvent): void => {
            if (compileKeyCodes.includes(e.keyCode) && e.ctrlKey && e.altKey) {
                handler();
                e.preventDefault();
            }
        };

        document.addEventListener('keydown', keyHandler);

        return () => {
            document.removeEventListener('keydown', keyHandler);
        };
    }, [handler, compileKeyCodes]);
};
