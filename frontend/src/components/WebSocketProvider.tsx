import React, { useEffect } from 'react';
import { useWebSocket } from '../services/websocket';

interface WebSocketProviderProps {
    children: React.ReactNode;
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({ children }) => {
    const { connect, disconnect, isConnected, error } = useWebSocket();

    useEffect(() => {
        connect();
        return () => {
            disconnect();
        };
    }, [connect, disconnect]);

    if (error) {
        console.error('WebSocket error:', error);
    }

    return (
        <div>
            {!isConnected && (
                <div className="fixed top-0 left-0 right-0 bg-yellow-100 text-yellow-800 p-2 text-center">
                    Connecting to WebSocket...
                </div>
            )}
            {children}
        </div>
    );
}; 