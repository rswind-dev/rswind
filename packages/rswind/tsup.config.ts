import { execSync } from 'node:child_process'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

import type { Options } from 'tsup'
import { defineConfig } from 'tsup'

const generateNamedExportPlugin: Required<Options>['esbuildPlugins'][0] = {
  name: 'rswind:generate-named-export',
  setup(build) {
    if (build.initialOptions.entryPoints?.length !== 1) {
      throw new Error('entryPoints must have exactly one item')
    }

    const entry = resolve(__dirname, build.initialOptions.entryPoints[0])
    const entryRe = new RegExp(`^${entry}$`)

    build.onLoad({ filter: entryRe }, async (_args) => {
      const output = execSync('../../scripts/generate-binding.ts', {
        stdio: 'pipe',
      })

      return {
        contents: `${readFileSync(entry)}\n${output.toString()}`,
        loader: 'ts',
      }
    })
  },
}

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['esm', 'cjs'],
  target: 'node16',
  inject: ['src/esm-shims.ts'],
  esbuildPlugins: [
    generateNamedExportPlugin,
  ],
  clean: true,
  dts: true,
  shims: true,
})
