import { create } from 'zustand';

interface WebSocketState {
    socket: WebSocket | null;
    isConnected: boolean;
    error: string | null;
    connect: () => void;
    disconnect: () => void;
    sendMessage: (message: string) => void;
}

export const useWebSocket = create<WebSocketState>((set, get) => ({
    socket: null,
    isConnected: false,
    error: null,
    connect: () => {
        const wsUrl = import.meta.env.VITE_API_BASE_URL?.replace('http', 'ws') || 'ws://localhost:8080/ws';
        const socket = new WebSocket(wsUrl);

        socket.onopen = () => {
            set({ isConnected: true, error: null });
            console.log('WebSocket connected');
        };

        socket.onclose = () => {
            set({ isConnected: false, socket: null });
            console.log('WebSocket disconnected');
        };

        socket.onerror = (error) => {
            set({ error: 'WebSocket error', isConnected: false });
            console.error('WebSocket error:', error);
        };

        set({ socket });
    },
    disconnect: () => {
        const { socket } = get();
        if (socket) {
            socket.close();
            set({ socket: null, isConnected: false });
        }
    },
    sendMessage: (message: string) => {
        const { socket, isConnected } = get();
        if (socket && isConnected) {
            socket.send(message);
        } else {
            console.error('Cannot send message: WebSocket is not connected');
        }
    },
})); 