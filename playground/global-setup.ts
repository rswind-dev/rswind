import path from 'node:path'
import process from 'node:process'
import crypto from 'node:crypto'
import fs from 'fs-extra'
import type { GlobalSetupContext } from 'vitest/node'
import type { BrowserServer } from 'playwright-chromium'
import { chromium } from 'playwright-chromium'

let browserServer: BrowserServer | undefined

const tempDir = path.resolve(__dirname, `../playground-temp-${crypto.randomUUID()}`)

export async function setup({ provide }: GlobalSetupContext): Promise<void> {
  browserServer = await chromium.launchServer({
    args: process.env.CI
      ? ['--no-sandbox', '--disable-setuid-sandbox']
      : undefined,
  })

  provide('wsEndpoint', browserServer.wsEndpoint())
  provide('tempDir', tempDir)

  await fs.ensureDir(tempDir)
  await fs.emptyDir(tempDir)
  await fs
    .copy(path.resolve(__dirname, '../playground'), tempDir, {
      dereference: false,
      filter(file) {
        file = file.replace(/\\/g, '/')
        return !file.includes('__tests__') && !/dist(?:\/|$)/.test(file)
      },
    })
    .catch((error) => {
      if (error.code === 'EPERM' && error.syscall === 'symlink') {
        throw new Error(
          'Could not create symlinks. On Windows, consider activating Developer Mode to allow non-admin users to create symlinks by following the instructions at https://docs.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development.',
        )
      }
      else {
        throw error
      }
    })
}

export async function teardown(): Promise<void> {
  await browserServer?.close()
  fs.removeSync(tempDir)
}

declare module 'vitest' {
  export interface ProvidedContext {
    wsEndpoint: string
    tempDir: string
  }
}
