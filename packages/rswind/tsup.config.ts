import { defineConfig } from 'tsup'

// const generateNamedExportPlugin: Required<Options>['esbuildPlugins'][0] = {
//   name: 'rswind:generate-named-export',
//   setup(build) {
//     if (build.initialOptions.entryPoints?.length !== 1) {
//       throw new Error('entryPoints must have exactly one item')
//     }

//     const entry = resolve(__dirname, build.initialOptions.entryPoints[0])
//     const entryRe = new RegExp(`^${entry}$`)

//     build.onLoad({ filter: entryRe }, async (_args) => {
//       const tempPath = tempfile()
//       writeFileSync(tempPath, '')

//       execSync(`../../scripts/generate-binding.ts ${tempPath}`)

//       return {
//         contents: await Promise.all([
//           readFile(tempPath),
//           readFile(entry),
//         ]).then(([generated, original]) => {
//           return `${generated}\n${original}`
//         }),
//         loader: 'ts',
//       }
//     })
//   },
// }

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['esm', 'cjs'],
  target: 'node16',
  inject: ['src/esm-shims.ts'],
  esbuildPlugins: [
    // generateNamedExportPlugin,
  ],
  clean: true,
  dts: {
    entry: 'src/index.ts',
  },
  shims: true,
})
