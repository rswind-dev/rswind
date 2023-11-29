import type { Plugin, ViteDevServer } from 'vite'
import { gen } from 'arrowcss'


function sendUpdate(server: ViteDevServer, id: string) {
  console.log('sendUpdate', id)
  const mod = server.moduleGraph.getModuleById(id);
  if (!mod) {
    return;
  }
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

export default function arrowCSSPlugin(): Plugin[] {
  let modules = new Map();
  let server: ViteDevServer | null = null;
  let entry = '';
  return [
    {
      name: "arrowcss:pre",
      enforce: 'pre',
      transform(code, id) {
        modules.set(id, code);
        if (server) {
          sendUpdate(server, entry)
        }
      },
      load(id) {
        if (id.includes("arrow.css")) {
          entry = id;
          server && sendUpdate(server, id);
          const res = gen([...modules.values()].join('\n') + '<div class="flex">');
          console.log(res)
          return res
        }
      },
    },
    {
      name: "arrowcss",
      enforce: 'post',
      configureServer(_server) {
        server = _server;

        _server.ws.on('arrow:hmr', async (msg) => {
          console.log('arrow:hmr', msg)
        });
      },
      resolveId(id) {
        if (id.endsWith("arrow.css")) {
          console.log({ id })
          return id;
        }
      },
      transform(code, id) {
        modules.set(id, code);
        if (id.includes("arrow.css")) {
          const hmr = `
        import.meta.hot.send('arrow:hmr', ['hello']);
        `
          return {
            code: code + hmr,
          }
        }
      },
    },
  ]
}