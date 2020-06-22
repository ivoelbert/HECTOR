import React, { ErrorInfo } from 'react';

interface ErrorBoundaryProps {
    fallback: React.ReactNode;
    children: React.ReactNode;
}

interface ErrorBoundaryState {
    error: Error | null;
}

export class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
    readonly state = {
        error: null,
    };

    static getDerivedStateFromError(error: Error) {
        return { error };
    }

    componentDidCatch(error: Error, errorInfo: ErrorInfo) {
        console.error(error, errorInfo);
    }

    render(): JSX.Element {
        if (this.state.error !== null) {
            return <>{this.props.fallback}</>;
        }

        return <>{this.props.children}</>;
    }
}
