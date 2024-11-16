import type { createGenerator } from 'rswind'
import type { ViteDevServer } from 'vite'

export interface ModulesQueue {
  server?: ViteDevServer
  generator: ReturnType<typeof createGenerator>
  modules: Map<string, string>
  css: string
  push: (id: string, code: string) => void
  flush: (length?: number) => void
}

export function createModulesQueue(
  generator: ReturnType<typeof createGenerator>,
  callback: (queue: ModulesQueue) => void,
): ModulesQueue {
  const modules = new Map<string, string>()
  let timer: NodeJS.Timeout | undefined

  return {
    generator,
    modules,
    css: '',
    push(id, code) {
      modules.set(id, code)
      clearTimeout(timer)
      timer = setTimeout(() => this.flush(), 0)
    },
    flush(length) {
      if (modules.size === 0 && (length === undefined || length === this.css.length)) {
        return
      }
      callback(this)
    },
  }
}
