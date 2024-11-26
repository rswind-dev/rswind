import { defineConfig } from 'vitepress'
import packageJson from '../../package.json'

export default defineConfig({
  title: 'Rswind',
  description: 'Rswind is a fast Tailwind CSS JIT engine in Rust.',
  lastUpdated: true,
  themeConfig: {
    logo: '/logo.png',
    search: {
      provider: 'local',
    },
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'API', link: '/api-reference' },
      { text: 'Config', link: '/config' },
      {
        text: `v${packageJson.version}`,
        items: [
          {
            text: 'Rust Docs',
            link: 'https://docs.rs/rswind',
          },
        ],
      },
    ],
    sidebar: {
      '/guide': [
        {
          text: 'Guides',
          items: [
            {
              text: 'Getting Started',
              link: '/guide/getting-started',
            },
            {
              text: 'Command Line Interface',
              link: '/guide/cli',
            },
            {
              text: 'Wasm Runtime',
              link: '/guide/wasm',
            },
          ],
        },
      ],
      '/api': [
        {
          text: 'API Reference',
          items: [
            {
              text: 'JavaScript API',
              link: '/api-reference',
            },
          ],
        },
      ],
      '/config': [
        {
          text: 'Getting Started',
          link: '/config',
        },
        {
          text: 'Advanced Configuration',
          link: '/config/advanced',
        },
      ],
    },
    socialLinks: [
      { icon: 'npm', link: 'https://www.npmjs.com/package/rswind' },
      { icon: 'github', link: 'https://github.com/rswind-dev/rswind' },
    ],
    editLink: {
      pattern: 'https://github.com/rswind-dev/rswind/edit/main/docs/:path',
      text: 'Suggest changes to this page',
    },
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present Samuel Lyon',
    },
  },
})
