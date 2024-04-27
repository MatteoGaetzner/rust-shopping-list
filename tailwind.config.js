/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        "./frontend/src/**/*.{rs,html,css}",
        "./frontend/dist/**/*.html"
    ],
    theme: {
        extend: {},
    },
    plugins: [require("daisyui")],
    daisyui: {
        themes: [
            "cupcake",
        ],
    },
}

