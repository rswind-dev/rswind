import antfu from '@antfu/eslint-config'

export default antfu({
  toml: {
    overrides: {
      'toml/padding-line-between-pairs': 'off',
      'toml/array-element-newline': 'off',
      'toml/tables-order': 'off',
    },
  },
  rules: {
    'eslint-comments/no-unlimited-disable': 'off',
  },
}, [
  {
    name: 'disable-generated-wasm-bindings',
    ignores: ['packages/wasm-runtime/src/binding/*'],
  },
])
