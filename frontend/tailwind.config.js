/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
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
                gray: {
                    50: "#f9fafb",
                    100: "#f3f4f6",
                    200: "#e5e7eb",
                    300: "#d1d5db",
                    400: "#9ca3af",
                    500: "#6b7280",
                    600: "#4b5563",
                    700: "#374151",
                    800: "#1f2937",
                    900: "#111827",
                },
            },
            fontFamily: {
                sans: [
                    "-apple-system",
                    "BlinkMacSystemFont",
                    '"Segoe UI"',
                    "Roboto",
                    '"Helvetica Neue"',
                    "Arial",
                    "sans-serif",
                ],
            },
            boxShadow: {
                card: "0 2px 8px rgba(0,0,0,0.1)",
            },
            borderRadius: {
                card: "0.5rem",
            },
        },
    },
    plugins: [],
}; 