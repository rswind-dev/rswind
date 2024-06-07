import { createRequire } from 'node:module'

if (TSUP_FORMAT === 'esm') {
  globalThis.require = createRequire(import.meta.url)
}
