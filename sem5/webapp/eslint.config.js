import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";
import { defineConfig, globalIgnores } from "eslint/config";
// highlight-start
import jsdoc from "eslint-plugin-jsdoc";
import tsdoc from "eslint-plugin-tsdoc";
// highlight-end

export default defineConfig([
  globalIgnores(["dist"]),
  {
    files: ["**/*.{ts,tsx}"],
    extends: [
      js.configs.recommended,
      ...tseslint.configs.recommended, // Note: typescript-eslint configs usually need a spread (...) operator in flat config arrays
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    // highlight-start
    plugins: {
      jsdoc,
      tsdoc,
    },
    rules: {
      // 1. Force documentation blocks to exist on your public functions/classes
      "jsdoc/require-jsdoc": [
        "error",
        {
          publicOnly: true,
          require: {
            FunctionDeclaration: true,
            MethodDefinition: true,
            ClassDeclaration: true,
            ArrowFunctionExpression: true,
          },
        },
      ],
      // 2. Enforce that those comments follow the strict TSDoc syntax specification
      "tsdoc/syntax": "error",
    },
    // highlight-end
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
]);
