import process from 'node:process'
import { defineConfig } from 'vitest/config'

const timeout = process.env.CI ? 50000 : 30000

const ignoreLog = [
  /^Re-optimizing dependencies because lockfile has changed$/,
  /^Port \d+ is in use, trying another one...$/,
  /\[vite\] hmr update/,
]

export default defineConfig({
  test: {
    include: ['./playground/**/*.spec.[tj]s'],
    globalSetup: ['./playground/global-setup.ts'],
    testTimeout: timeout,
    hookTimeout: timeout,
    reporters: 'dot',
    onConsoleLog(msg, type) {
      if (type === 'stderr') {
        return true
      }
      return ignoreLog.some(re => re.test(msg))
    },
  },
  esbuild: {
    target: 'node18',
  },
  publicDir: false,
})
