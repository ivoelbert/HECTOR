import React, { useState, useCallback } from 'react';
import { TabProps } from './Tab';
import { isNil } from '../../utils/utils';
import { useCtrlAltKeys } from '../../hooks/useCtrlAltKeys';

interface TabsProps {
    children: React.ReactNode;
}

// Should only receive Tab components
export const Tabs: React.FC<TabsProps> = (props) => {
    const [selectedIndex, setSelectedIndex] = useState(0);

    const names = React.Children.map(props.children, (child) => {
        if (!child) {
            throw new Error('Tabs component must have Tab children');
        }

        const anyChild = child as any;
        if (!anyChild.props) {
            throw new Error('Tabs component must have Tab children');
        }

        const props = anyChild.props as TabProps;
        if (!props.name) {
            throw new Error('Tabs component must have Tab children');
        }

        return props.name;
    });

    if (isNil(names)) {
        throw new Error('Tabs component must have Tab children');
    }

    const tabsArray = React.Children.toArray(props.children);
    const visibleTab = tabsArray[selectedIndex];

    return (
        <div className="tabs-container">
            <TabsButtons
                names={names}
                selectedIndex={selectedIndex}
                setSelectedIndex={setSelectedIndex}
            />
            {visibleTab}
        </div>
    );
};

interface TabsButtonsProps {
    names: string[];
    selectedIndex: number;
    setSelectedIndex: React.Dispatch<React.SetStateAction<number>>;
}
const TabsButtons: React.FC<TabsButtonsProps> = ({ names, selectedIndex, setSelectedIndex }) => {
    return (
        <div className="tabs-buttons">
            {names.map((name, index) => (
                <TabButton
                    key={name}
                    name={name}
                    index={index}
                    selectedIndex={selectedIndex}
                    setSelectedIndex={setSelectedIndex}
                />
            ))}
        </div>
    );
};

interface TabButtonProps {
    name: string;
    index: number;
    selectedIndex: number;
    setSelectedIndex: React.Dispatch<React.SetStateAction<number>>;
}

const TabButton: React.FC<TabButtonProps> = ({ name, index, selectedIndex, setSelectedIndex }) => {
    const selectedClass = index === selectedIndex ? 'selected' : '';

    const moveToTab = useCallback(() => {
        setSelectedIndex(index);
    }, []);
    useCtrlAltKeys([49 + index], moveToTab);

    return (
        <button key={name} className={selectedClass} onClick={moveToTab}>
            {name}
        </button>
    );
};
