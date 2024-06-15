declare module 'vite' {
  interface ViteDevServer {
    _currentServerPort: number
  }
}

export {}
