import { defineConfig } from 'tsup'

export default defineConfig({
  format: ['esm', 'cjs', 'iife'],
  target: 'es2015',
  loader: { '.wasm': 'file' },
  define: {
    'import.meta.url': '{}',
  },
  minify: true,
  clean: false,
  dts: true,
  noExternal: ['@rswind/binding_core_wasm'],
})