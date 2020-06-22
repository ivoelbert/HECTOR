import React from 'react';
import { useWatFromWasm } from '../../hooks/useWatFromWasm';
import './WASMViewer.scss';

interface WASMViewerProps {
    bin: any;
}

export const WASMViewer: React.FC<WASMViewerProps> = ({ bin }) => {
    const wat = useWatFromWasm(bin);
    return (
        <div className="wat-container">
            <pre>{wat}</pre>
        </div>
    );
};
