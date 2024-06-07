import antfu from '@antfu/eslint-config'

export default antfu({
  toml: {
    overrides: {
      'toml/padding-line-between-pairs': 'off',
    },
  },
  rules: {
    'eslint-comments/no-unlimited-disable': 'off',
  },
}, [
  {
    name: 'disable-generated-wasm-bindings',
    ignores: ['packages/rswind-wasm-web/src/binding/*'],
  },
])
