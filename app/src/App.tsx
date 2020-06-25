import React from 'react';
import { Compiler } from './components/Compiler/Compiler';
import { Header } from './components/Header/Header';
import { Footer } from './components/Footer.tsx/Footer';

const App: React.FC = () => {
    return (
        <div className="app">
            <div>
                <Header />
                <Compiler />
            </div>
            <Footer />
        </div>
    );
};

export default App;
