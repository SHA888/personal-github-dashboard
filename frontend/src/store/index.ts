import { configureStore } from '@reduxjs/toolkit';
import repositoryReducer from './slices/repositorySlice';
import analyticsReducer from './slices/analyticsSlice';
import websocketReducer from './slices/websocketSlice';

export const store = configureStore({
    reducer: {
        repository: repositoryReducer,
        analytics: analyticsReducer,
        websocket: websocketReducer,
    },
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: {
                // Ignore these action types
                ignoredActions: ['websocket/setSocket'],
                // Ignore these field paths in all actions
                ignoredActionPaths: ['payload.socket'],
                // Ignore these paths in the state
                ignoredPaths: ['websocket.socket'],
            },
        }),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch; 