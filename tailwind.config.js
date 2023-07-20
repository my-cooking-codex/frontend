module.exports = {
    content: [
        "./src/**/*.rs"
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require("daisyui"),
    ],
    daisyui: {
        themes: [
            {
                light: {
                    ...require("daisyui/src/theming/themes")["[data-theme=light]"],
                    "primary": "#2a76b7",
                },
                dark: {
                    ...require("daisyui/src/theming/themes")["[data-theme=dark]"],
                    "primary": "#235886",
                },
            },
        ],
    },
}
