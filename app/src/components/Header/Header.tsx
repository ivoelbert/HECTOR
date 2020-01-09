import React from 'react';
import './Header.scss';

export const Header: React.FC = () => {
    return (
        <header className="header-container">
            <h1 className="title">HECTOR</h1>
            <span className="description">Heuristically Excessive Compiler for Tiger On Rust</span>
        </header>
    )
}