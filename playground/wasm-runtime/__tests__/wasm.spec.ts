import http from 'node:http'
import { beforeAll, describe, expect, it } from 'vitest'
import sirv from 'sirv'
import type { TestContext } from '../../test-utils'
import { colorOf, setupContext, untilUpdated } from '../../test-utils'

let ctx: TestContext<false>
let port: number
beforeAll(async (s) => {
  ctx = await setupContext(s, false)
  port = await setupStaticServer(ctx.testDir)
})

function setupStaticServer(base: string): Promise<number> {
  const assets = sirv(base)
  const httpServer = http.createServer((req, res) => {
    assets(req, res, () => {
      res.statusCode = 404
      res.end('Not found')
    })
  })

  let port = 3001
  return new Promise((resolve, reject) => {
    const onError = (e: Error & { code?: string }) => {
      if (e.code === 'EADDRINUSE') {
        httpServer.listen(++port)
      }
      else {
        httpServer.removeListener('error', onError)
        reject(e)
      }
    }

    httpServer.on('error', onError)

    httpServer.listen(port, () => {
      httpServer.removeListener('error', onError)
      resolve(port)
    })
  })
}

describe('rswind wasm runtime', () => {
  it('runs with update', async () => {
    const { page } = ctx
    await page.goto(`http://localhost:${port}/index.html`)

    const tailwindDiv = page.locator('#tailwind-div')

    await untilUpdated(() => colorOf(tailwindDiv), 'rgb(59, 130, 246)')

    expect(await colorOf(tailwindDiv)).toBe('rgb(59, 130, 246)')

    tailwindDiv.evaluate((e) => {
      e.classList.remove('text-blue-500')
      e.classList.add('text-red-500')
    })

    await untilUpdated(() => colorOf(tailwindDiv), 'rgb(239, 68, 68)')

    expect(await colorOf(tailwindDiv)).toBe('rgb(239, 68, 68)')
  })
})
