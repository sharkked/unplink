import eslint from '@eslint/js'
import eslintConfigPrettier from 'eslint-config-prettier/flat'
import eslintPluginVue from 'eslint-plugin-vue'
import eslintPluginVitest from '@vitest/eslint-plugin'
import globals from 'globals'
import typescriptEslint from 'typescript-eslint'
import vueParser from 'vue-eslint-parser'

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore

export default typescriptEslint.config([
  eslint.configs.recommended,
  typescriptEslint.configs.recommended,
  eslintPluginVue.configs['flat/recommended'],
  {
    ...eslintPluginVitest.configs.recommended,
    files: ['src/**/__tests__/*'],
  },
  {
    files: ['*.vue', '**/*.vue'],
    rules: {},
    languageOptions: {
      globals: {
        ...globals.browser,
      },
      parser: vueParser,
      parserOptions: {
        parser: typescriptEslint.parser,
        extraFileExtensions: ['.vue'],
        sourceType: 'module',
      },
    },
  },
  {
    files: ['src/pages/**/*.vue'],
    rules: {
      'vue/multi-word-component-names': ['off'],
    },
  },
  {
    ignores: ['env.d.ts'],
  },
  eslintConfigPrettier,
])
