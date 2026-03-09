import js from "@eslint/js";
import stylisticJs from "@stylistic/eslint-plugin-js";
import globals from "globals";

export default [
    js.configs.recommended,
    {
        files: ["src/**/*.js"],
        languageOptions: {
            ecmaVersion: "latest",
            sourceType: "module",
            globals: {
                ...globals.browser,
                ...globals.node,
            },
        },
        plugins: {
            "@stylistic/js": stylisticJs,
        },
        rules: {
            "indent": ["error", 4],
            "linebreak-style": ["error", "unix"],
            "quotes": ["error", "double"],
            "semi": ["error", "always"],
            "no-unused-vars": ["error", { "argsIgnorePattern": "^_" }],
        },
    },
    {
        ignores: ["node_modules/", "src/wasm/"],
    },
];
