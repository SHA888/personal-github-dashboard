import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface WebSocketState {
  socket: WebSocket | null;
  isConnected: boolean;
  error: string | null;
  messages: any[];
}

const initialState: WebSocketState = {
  socket: null,
  isConnected: false,
  error: null,
  messages: [],
};

const websocketSlice = createSlice({
  name: 'websocket',
  initialState,
  reducers: {
    setSocket: (state, action: PayloadAction<WebSocket>) => {
      state.socket = action.payload;
    },
    setConnected: (state, action: PayloadAction<boolean>) => {
      state.isConnected = action.payload;
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload;
    },
    addMessage: (state, action: PayloadAction<any>) => {
      state.messages.push(action.payload);
    },
    clearMessages: (state) => {
      state.messages = [];
    },
  },
});

export const { setSocket, setConnected, setError, addMessage, clearMessages } = websocketSlice.actions;
export default websocketSlice.reducer; 