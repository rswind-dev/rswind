import process from 'node:process'
import path, { resolve } from 'node:path'
import fs from 'node:fs'
import { type File, type Suite, inject } from 'vitest'
import { type Locator, type Page, chromium } from 'playwright-chromium'
import type { PreviewServer, ViteDevServer } from 'vite'
import { setupViteServer } from './setup-vite'

export const workspaceRoot = resolve(__dirname, '..')

export const isBuild = !!process.env.RSWIND_VITE_TEST_BUILD

export function slash(p: string): string {
  return p.replace(/\\/g, '/')
}

export type TestContext<Vite extends boolean> = Vite extends true
  ? { vite: ViteDevServer | PreviewServer, page: Page, testDir: string }
  : { page: Page, testDir: string }

export async function setupContext<Vite extends boolean>(
  suite: Readonly<File | Suite>,
  needVite: Vite,
): Promise<TestContext<Vite>> {
  const testPath = suite.filepath!
  const testName = slash(testPath).match(/playground\/([\w-]+)\//)?.[1]
  if (!testName) {
    throw new Error('Tests must stay in playground')
  }

  const testDir = path.resolve(inject('tempDir'), testName)

  const browser = await chromium.connect(inject('wsEndpoint'))
  const page = await browser.newPage()

  return {
    page,
    testDir,
    vite: needVite ? await setupViteServer(testDir, page) : undefined,
  } as TestContext<Vite>
}

export async function colorOf(locator: Locator) {
  return locator.evaluate(el => getComputedStyle(el).color)
}

export async function untilUpdated<R>(
  poll: () => R | Promise<R>,
  expected: R,
  timeout = 5000,
) {
  let lastResult: R | undefined
  return Promise.race([
    new Promise<void>((resolve) => {
      const interval = setInterval(async () => {
        const result = await poll()
        lastResult = result
        if (result === expected) {
          clearInterval(interval)
          resolve()
        }
      }, 100)
    }),
    new Promise((_, rej) =>
      setTimeout(() =>
        rej(
          new Error(
            `Poll Timeout in \`untilUpdated\`: waiting for "${poll}" to equal "${expected}" didn\'t complete in ${timeout} ms,\nLast result: ${lastResult}
            `,
          ),
        ), timeout),
    ),
  ])
}

export function editFile(file: string, modifyFn: (code: string) => string) {
  const content = fs.readFileSync(file, 'utf-8')
  const modified = modifyFn(content)
  fs.writeFileSync(file, modified)
}
