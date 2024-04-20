import type { Plugin, ViteDevServer } from 'vite'
import { createApp } from 'arrowcss'

let app = createApp()

function sendUpdate(server: ViteDevServer) {
  const id = '/__arrow.css';
  const mod = server.moduleGraph.getModuleById(id);
  if (!mod) {
    return;
  }
  console.log('sendUpdate', id)
  server.moduleGraph.invalidateModule(mod);
  setTimeout(() => {
    server.ws.send({
      type: "update",
      updates: [{
        acceptedPath: mod.url,
        path: mod.url,
        type: "js-update",
        timestamp: Date.now(),
      }]
    });
  }, 10)
}

export default function arrowCSSPlugin(): Plugin[] {
  let modules: Map<string, string> = new Map();
  let server: ViteDevServer | null = null;
  let entry = '';
  return [
    {
      name: "arrowcss:pre",
      enforce: 'pre',
      transform(code, id) {
        if (id.includes("arrow.css")) {
          return null
        }
        if (modules.get(id) !== code) {
          modules.set(id, code);
          // console.log(modules.keys())
          server && sendUpdate(server);
        }
      },
      load(id) {
        if (id.includes("arrow.css")) {
          entry = id;
          return app.generate([...modules.values()].join('\n'));
        }
      },
    },
    {
      name: "arrowcss",
      enforce: 'post',
      configureServer(_server) {
        server = _server;

        _server.ws.on('arrow:hmr', async (msg) => {
          // console.log('arrow:hmr', msg)
        });
      },
      resolveId(id) {
        if (id.endsWith("arrow.css")) {
          // console.log({ id })
          return '/__arrow.css'
        }
      },
      transform(code, id) {
        if (id.includes("arrow.css")) {
          const hmr = `
        // import.meta.hot.send('arrow:hmr', ['hello']);
        `
          return {
            code: code + hmr,
          }
        }
      },
    },
  ]
}