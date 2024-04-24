import type { Plugin, ViteDevServer } from 'vite'
import { createApp } from 'arrowcss'

let app = createApp()

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

export default function arrowcssPlugin(): Plugin[] {
  let modulesQueue: Map<string, string> = new Map();
  let server: ViteDevServer | null = null;
  return [
    {
      name: "arrowcss:pre",
      enforce: 'pre',
      configureServer(_server) {
        server = _server;
      },
      transform(code, id) {
        if (id.includes("arrow.css")) {
          return null
        }
        if (modulesQueue.get(id) !== code) {
          modulesQueue.set(id, code);
          server && sendUpdateDebounced(server);
        }
      },
      load(id) {
        if (id.includes("arrow.css")) {
          let res = app.generate([...modulesQueue.values()].join('\n'));
          modulesQueue.clear();
          return res;
        }
      },
      resolveId(id) {
        if (id.endsWith("arrow.css")) {
          return '/__arrow.css'
        }
      },
    },
  ]
}