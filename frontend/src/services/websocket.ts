import { useEffect, useRef, useCallback } from 'react';

interface WebSocketMessage {
    type: string;
    data: any;
}

interface WebSocketService {
    connect: () => void;
    disconnect: () => void;
    subscribe: (type: string, callback: (data: any) => void) => void;
    unsubscribe: (type: string) => void;
}

class WebSocketService implements WebSocketService {
    private ws: WebSocket | null = null;
    private subscribers: Map<string, Set<(data: any) => void>> = new Map();
    private reconnectAttempts = 0;
    private maxReconnectAttempts = 5;
    private reconnectTimeout = 1000;

    constructor(private url: string) { }

    connect() {
        if (this.ws) return;

        this.ws = new WebSocket(this.url);

        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.reconnectAttempts = 0;
        };

        this.ws.onmessage = (event) => {
            try {
                const message: WebSocketMessage = JSON.parse(event.data);
                const callbacks = this.subscribers.get(message.type);
                if (callbacks) {
                    callbacks.forEach(callback => callback(message.data));
                }
            } catch (error) {
                console.error('Error processing WebSocket message:', error);
            }
        };

        this.ws.onclose = () => {
            console.log('WebSocket disconnected');
            this.ws = null;
            this.attemptReconnect();
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    }

    disconnect() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }

    subscribe(type: string, callback: (data: any) => void) {
        if (!this.subscribers.has(type)) {
            this.subscribers.set(type, new Set());
        }
        this.subscribers.get(type)?.add(callback);
    }

    unsubscribe(type: string) {
        this.subscribers.delete(type);
    }

    private attemptReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            setTimeout(() => {
                console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
                this.connect();
            }, this.reconnectTimeout * this.reconnectAttempts);
        }
    }
}

// Create a singleton instance
const wsService = new WebSocketService('ws://localhost:8080/ws');

// React hook for using WebSocket
export const useWebSocket = (type: string, callback: (data: any) => void) => {
    const callbackRef = useRef(callback);

    useEffect(() => {
        callbackRef.current = callback;
    }, [callback]);

    useEffect(() => {
        const handleMessage = (data: any) => {
            callbackRef.current(data);
        };

        wsService.subscribe(type, handleMessage);
        wsService.connect();

        return () => {
            wsService.unsubscribe(type);
        };
    }, [type]);
};

export default wsService; 