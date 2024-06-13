import { defineConfig } from 'tsup'

export default defineConfig({
  target: 'es2015',
  loader: { '.wasm': 'file' },
  define: {
    'import.meta.url': '{}',
  },
  minify: true,
  clean: false,
  dts: false,
  noExternal: ['@rswind/binding_core_wasm'],
})
