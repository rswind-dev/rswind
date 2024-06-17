import { build, createServer, preview } from 'vite'
import type { Page } from 'playwright-chromium'
import { isBuild } from './test-utils'

export async function setupViteServer(root: string, page: Page) {
  return isBuild
    ? setupVitePreviewServer(root, page)
    : setupViteDevServer(root, page)
}

async function setupVitePreviewServer(root: string, page: Page) {
  await build({ root })
  const previewServer = await preview({ root })

  await page.goto(previewServer.resolvedUrls.local[0])

  return previewServer
}

async function setupViteDevServer(root: string, page: Page) {
  const devServer = await (await createServer({ root })).listen()

  await page.goto(devServer.resolvedUrls.local[0])
  return devServer
}
