import React from 'react';
import './Footer.scss';

export const Footer: React.FC = () => {
    return (
        <footer className="footer-container">
            Made with <Rage /> and <Blood /> by <IvoLink /> and <FedeLink />
        </footer>
    );
};

const Blood: React.FC = () => {
    return (
        <span role="img" aria-label="blood" className="blood emoji">
            🩸
        </span>
    );
};

const Rage: React.FC = () => {
    return (
        <span role="img" aria-label="rage" className="rage emoji">
            😡
        </span>
    );
};

const IvoLink: React.FC = () => {
    return (
        <a
            className="link"
            href="https://github.com/ivoelbert"
            target="_blank"
            rel="noopener noreferrer"
        >
            Ivo Elbert
        </a>
    );
};

const FedeLink: React.FC = () => {
    return (
        <a
            className="link"
            href="https://github.com/QPotato"
            target="_blank"
            rel="noopener noreferrer"
        >
            Fede Badaloni
        </a>
    );
};
