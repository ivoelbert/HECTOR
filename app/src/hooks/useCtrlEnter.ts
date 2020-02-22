import { useEffect } from 'react';

export const useCtrlEnter = (handler: () => void): void => {
    useEffect(() => {
        const keyHandler = (e: KeyboardEvent): void => {
            const compileKeyCodes = [13, 83];

            if (compileKeyCodes.includes(e.keyCode) && e.ctrlKey) {
                handler();
                e.preventDefault();
            }
        };

        document.addEventListener('keydown', keyHandler);

        return () => {
            document.removeEventListener('keydown', keyHandler);
        };
    }, [handler]);
};
