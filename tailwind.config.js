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
                    "secondary": "#2b7c95",
                    "primary-content": "white",
                    "secondary-content": "white",
                },
                dark: {
                    ...require("daisyui/src/theming/themes")["[data-theme=dark]"],
                    "primary": "#235886",
                    "secondary": "#236686",
                    "primary-content": "#cfcfcf",
                    "secondary-content": "#cfcfcf",
                },
            },
        ],
    },
}
