import { resolve } from 'node:path'
import { createServer } from 'vite'
import rswind from '@rswind/vite'

export async function setupViteServer(root: string): Promise<number> {
  const devServer = await (await createServer({
    root,
    plugins: [
      rswind(),
    ],
  })).listen()

  return devServer._currentServerPort
}
