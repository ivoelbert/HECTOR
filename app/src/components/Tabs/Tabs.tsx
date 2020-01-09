import React, { useState } from 'react';
import './Tabs.scss';

interface TabsProps {
    tabs: {
        [key: string]: JSX.Element;
    };
}
export const Tabs: React.FC<TabsProps> = ({ tabs }) => {
    const keys = Object.keys(tabs);

    const [visibleKey, setVisibleKey] = useState<string>(keys[0]);

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
