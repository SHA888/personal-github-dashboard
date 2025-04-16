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
    const baseUrl =
      import.meta.env.VITE_API_BASE_URL?.replace('/api', '') || 'http://localhost:8080';

    const wsUrl = baseUrl.replace(/^http/, 'ws') + '/ws';

    console.log('Connecting to WebSocket:', wsUrl);
    const socket = new WebSocket(wsUrl);

    socket.onopen = () => {
      set({ isConnected: true, error: null });
      console.log('WebSocket connected');
    };

    socket.onclose = (event) => {
      set({ isConnected: false, socket: null });
      console.log('WebSocket disconnected', event.code, event.reason);

      // Attempt to reconnect after 5 seconds
      setTimeout(() => {
        console.log('Attempting to reconnect...');
        get().connect();
      }, 5000);
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
