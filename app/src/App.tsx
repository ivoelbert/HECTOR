import React from 'react';
import { Compiler } from './components/Compiler/Compiler';
import { Header } from './components/Header/Header';

const App: React.FC = () => {
    return (
        <div className="App">
            <Header />
            <Compiler />
        </div>
    );
};

export default App;
