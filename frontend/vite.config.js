import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    server: {
        port: 5173,
        host: true,
        proxy: {
            '/api': {
                target: process.env.BACKEND_URL || 'http://localhost:3001',
                changeOrigin: true,
            },
            '/auth': {
                target: process.env.BACKEND_URL || 'http://localhost:3001',
                changeOrigin: true,
            },
        },
    },
});
