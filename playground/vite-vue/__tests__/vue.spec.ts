import { resolve } from 'node:path'
import { beforeAll, describe, expect, it } from 'vitest'

import type { Page } from 'playwright-chromium'
import { colorOf, editFile, isBuild, setupContext, untilUpdated } from '../../test-utils'

let page: Page
let testDir: string

beforeAll(async (s) => {
  const ctx = await setupContext(s, true)
  page = ctx.page
  testDir = ctx.testDir

  return async () => {
    await page.close()
    await ctx.vite.close()
  }
})

describe('rswind vite vue', () => {
  it('has tailwind style', async () => {
    const tailwindDiv = await page.waitForSelector('#tailwind-div')

    const style = await tailwindDiv.evaluate(el => getComputedStyle(el))

    expect(style.color).toMatchInlineSnapshot(`"rgb(59, 130, 246)"`)
    expect(style.display).toMatchInlineSnapshot(`"flex"`)
  })

  it.runIf(!isBuild)('handles hmr update', async () => {
    const tailwindDiv = page.locator('#tailwind-div')

    const file = resolve(testDir, 'src/App.vue')
    editFile(file, (code) => {
      return code.replace('text-blue-500', 'text-red-500')
    })
    await untilUpdated(() => colorOf(tailwindDiv), 'rgb(239, 68, 68)')

    const style = await tailwindDiv.evaluate(el => getComputedStyle(el))
    expect(style.color).toMatchInlineSnapshot(`"rgb(239, 68, 68)"`)
  })
})
