/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: "#2563eb",
                    light: "#3b82f6",
                    dark: "#1d4ed8",
                },
                secondary: {
                    DEFAULT: "#64748b",
                    light: "#94a3b8",
                    dark: "#475569",
                },
                success: {
                    DEFAULT: "#22c55e",
                    light: "#4ade80",
                    dark: "#16a34a",
                },
                warning: {
                    DEFAULT: "#f59e0b",
                    light: "#fbbf24",
                    dark: "#d97706",
                },
                danger: {
                    DEFAULT: "#ef4444",
                    light: "#f87171",
                    dark: "#dc2626",
                },
            },
            spacing: {
                '72': '18rem',
                '84': '21rem',
                '96': '24rem',
            },
            maxWidth: {
                '8xl': '88rem',
            },
            fontFamily: {
                sans: [
                    'Inter var',
                    '-apple-system',
                    'BlinkMacSystemFont',
                    'Segoe UI',
                    'Roboto',
                    'Oxygen',
                    'Ubuntu',
                    'Cantarell',
                    'Fira Sans',
                    'Droid Sans',
                    'Helvetica Neue',
                    'sans-serif',
                ],
            },
            fontSize: {
                'xs': ['0.75rem', { lineHeight: '1rem' }],
                'sm': ['0.875rem', { lineHeight: '1.25rem' }],
                'base': ['1rem', { lineHeight: '1.5rem' }],
                'lg': ['1.125rem', { lineHeight: '1.75rem' }],
                'xl': ['1.25rem', { lineHeight: '1.75rem' }],
                '2xl': ['1.5rem', { lineHeight: '2rem' }],
            },
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}; 