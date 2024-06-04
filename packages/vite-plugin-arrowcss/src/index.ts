import type { Plugin, ViteDevServer } from 'vite'
import { type GeneratorOptions, createApp } from 'rswind'

function debounce(fn: Function, delay: number) {
  let timer: NodeJS.Timeout;
  return function (...args: any) {
    clearTimeout(timer);
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  }
}

function sendUpdate(server: ViteDevServer) {
  const mod = server.moduleGraph.getModuleById('/__arrow.css');
  if (!mod) {
    return;
  }

  server.moduleGraph.invalidateModule(mod);
  server.ws.send({
    type: "update",
    updates: [{
      acceptedPath: mod.url,
      path: mod.url,
      type: "js-update",
      timestamp: Date.now(),
    }]
  });
}

const sendUpdateDebounced = debounce(sendUpdate, 10);

export default function rswindPlugin(options: GeneratorOptions): Plugin[] {
  let modulesQueue: [string, string][] = [];
  let server: ViteDevServer | null = null;
  let app = createApp(options);

  return [
    {
      name: "rswind:pre",
      enforce: 'pre',
      configureServer(_server) {
        server = _server;
      },
      resolveId(id) {
        if (id.endsWith("arrow.css")) {
          return '/__arrow.css'
        }
      },
      transform(code, id) {
        if (id.includes("arrow.css")) {
          return null
        }
        modulesQueue.push([id, code])
        server && sendUpdateDebounced(server);
      },
      load(id) {
        if (id.includes("arrow.css")) {
          let res = app.generateWith(modulesQueue);
          modulesQueue = [];
          return res;
        }
      },
    },
  ]
}