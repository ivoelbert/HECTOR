import React, { useState, useCallback } from 'react';
import './Tabs.scss';
import { useCtrlAltKeys } from '../../hooks/useCtrlAltKeys';

interface TabsProps {
    tabs: {
        [key: string]: JSX.Element;
    };
}
export const Tabs: React.FC<TabsProps> = ({ tabs }) => {
    const keys = Object.keys(tabs);

    const [visibleKey, setVisibleKey] = useState<string>(keys[0]);

    const moveTo1 = useCallback(() => {
        setVisibleKey(keys[0])
    }, [keys])
    useCtrlAltKeys([49], moveTo1)

    const moveTo2 = useCallback(() => {
        setVisibleKey(keys[1])
    }, [keys])
    useCtrlAltKeys([50], moveTo2)

    const moveTo3 = useCallback(() => {
        setVisibleKey(keys[2])
    }, [keys])
    useCtrlAltKeys([51], moveTo3)

    const moveTo4 = useCallback(() => {
        setVisibleKey(keys[3])
    }, [keys])
    useCtrlAltKeys([52], moveTo4)

    return (
        <div className="tabs-container">
            <TabsButtons keys={keys} visibleKey={visibleKey} setVisibleKey={setVisibleKey} />
            {tabs[visibleKey]}
        </div>
    );
};

interface TabsButtonsProps {
    keys: string[];
    visibleKey: string;
    setVisibleKey: React.Dispatch<React.SetStateAction<string>>;
}
const TabsButtons: React.FC<TabsButtonsProps> = ({ keys, visibleKey, setVisibleKey }) => {
    return (
        <div className="tabs-buttons">
            {keys.map(k => {
                const selectedClass = visibleKey === k ? 'selected' : '';
                return (
                    <button key={k} className={selectedClass} onClick={() => setVisibleKey(k)}>
                        {k}
                    </button>
                );
            })}
        </div>
    );
};
