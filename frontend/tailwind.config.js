/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{js,jsx,ts,tsx}",
        "./public/index.html",
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: "#1976d2",
                    light: "#42a5f5",
                    dark: "#1565c0",
                },
                secondary: {
                    DEFAULT: "#dc004e",
                    light: "#ff4081",
                    dark: "#9a0036",
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