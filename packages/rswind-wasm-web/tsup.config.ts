import { defineConfig } from 'tsup'
import { copy } from 'esbuild-plugin-copy'

export default defineConfig({
  target: 'es2022',
  loader: { '.wasm': 'file' },
  outDir: '.',
  esbuildPlugins: [
    copy({
      resolveFrom: 'cwd',
      assets: [
        {
          from: './src/binding/*.wasm',
          to: '.',
        },
      ],
    }),
  ],
  minify: true,
  clean: false,
  dts: false,
  noExternal: ['@rswind/binding_core_wasm'],
})
