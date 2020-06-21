import React from 'react';

export interface TabProps {
    name: string;
    children: React.ReactNode;
}

export const Tab: React.FC<TabProps> = (props) => {
    return <>{props.children}</>;
};
