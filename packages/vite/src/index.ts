import type { HookHandler, Plugin, ResolvedConfig, ViteDevServer } from 'vite'
import type { GeneratorOptions } from 'rswind'
import { createGenerator } from 'rswind'
import { createModulesQueue } from './modules-queue'

const WS_EVENT_PREFIX = 'rswind:hmr'
const RSWIND_PLACEHOLDER = '@rswind-base;'
const RSWIND_ID = '/__rswind.css'

export default function rswindPlugin(options: GeneratorOptions): Plugin[] {
  const generator = createGenerator(options)

  let server: ViteDevServer | null = null

  const modulesQueue = createModulesQueue(generator, (q) => {
    const res = generator.generateWith([...q.modules.entries()])
    if (res.kind === 'Cached')
      return
    q.css = res.css
    q.modules.clear()
    q.server && sendUpdate(q.server)
  })

  let cssPlugin: Plugin | undefined
  let cssPostPlugin: Plugin | undefined
  let viteConfig: ResolvedConfig

  return [
    {
      name: 'rswind:module-collector',
      enforce: 'pre',
      configureServer(_server) {
        server = _server
        modulesQueue.server = server

        server.ws.on(WS_EVENT_PREFIX, (length) => {
          modulesQueue.flush(length)
        })
      },
      buildStart() {
        // warm up the generator
        generator.generateCandidate([])
      },
      transform(code, id) {
        if (id === RSWIND_ID) {
          return null
        }
        modulesQueue.push(id, code)
      },
    },
    {
      name: 'rswind:post',
      apply: 'serve',
      enforce: 'post',
      resolveId(id) {
        if (id === 'rswind.css') {
          return RSWIND_ID
        }
      },
      load(id) {
        if (id === RSWIND_ID) {
          const res = modulesQueue.css || RSWIND_PLACEHOLDER

          return res
        }
      },
      transform(code, id) {
        if (id === RSWIND_ID) {
          const hmr = `;import.meta.hot && import.meta.hot.send('${WS_EVENT_PREFIX}', __vite__css.length);`
          return {
            code: code + hmr,
            map: null,
          }
        }
      },
    },
    {
      name: 'rswind:build',
      apply: 'build',
      enforce: 'pre',
      resolveId(id) {
        if (id === 'rswind.css') {
          return RSWIND_ID
        }
      },
      configResolved(config) {
        viteConfig = config
        cssPlugin = config.plugins.find(p => p.name === 'vite:css')
        cssPostPlugin = config.plugins.find(p => p.name === 'vite:css-post')
      },
      load(id) {
        if (id === RSWIND_ID) {
          return ''
        }
      },
      async renderChunk(code, chunk) {
        const fakeCssId = `${viteConfig.root}/${chunk.fileName}-rswind.css`

        modulesQueue.flush()
      
        const transformHandler = cssPlugin?.transform && getHookHandler(cssPlugin.transform)
        const postHandler = cssPostPlugin?.transform && getHookHandler(cssPostPlugin.transform)

        const res = transformHandler ? await transformHandler.call(this as any, modulesQueue.css, fakeCssId) : modulesQueue.css

        const css: string = typeof res !== 'string' && res != null ? res.code || modulesQueue.css : modulesQueue.css

        postHandler && await postHandler.call(this as any, css, fakeCssId)

        // inject the css to vite:css plugin, so it can be generated
        chunk.modules[fakeCssId] = {
          code: null,
          originalLength: 0,
          removedExports: [],
          renderedExports: [],
          renderedLength: 0,
        }
      },
    },
  ]
}

type ObjectHook<T, O = object> = T | ({ handler: T, order?: 'pre' | 'post' | null } & O)

export function getHookHandler<T extends ObjectHook<Function>>(
  hook: T,
): HookHandler<T> {
  return (typeof hook === 'object' ? hook.handler : hook) as HookHandler<T>
}

function sendUpdate(server: ViteDevServer) {
  const mod = server.moduleGraph.getModuleById(RSWIND_ID)
  if (!mod) {
    return
  }

  server.moduleGraph.invalidateModule(mod)
  server.ws.send({
    type: 'update',
    updates: [{
      acceptedPath: mod.url,
      path: mod.url,
      type: 'js-update',
      timestamp: Date.now(),
    }],
  })
}
