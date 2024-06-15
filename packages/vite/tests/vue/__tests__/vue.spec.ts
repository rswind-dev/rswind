import { resolve } from 'node:path'
import { expect, test } from '@playwright/test'
import { setupViteServer } from '../../setup-vite'

let port: number

test.beforeAll(async () => {
  port = await setupViteServer(resolve(import.meta.dirname, '..'))
})

test('has tailwind style', async ({ page }) => {
  await page.goto(`http://localhost:${port}/`)

  const tailwindDiv = page.locator('#tailwind-div')

  await expect(tailwindDiv).toHaveCSS('display', 'flex')
  await expect(tailwindDiv).toHaveCSS('color', 'rgb(59, 130, 246)')
})
