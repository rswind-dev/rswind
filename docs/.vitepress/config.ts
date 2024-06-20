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
      { text: 'Guide', link: '/getting-started' },
      { text: 'Config', link: '/configuration' },
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
    sidebar: [
      {
        text: 'Guides',
        items: [
          {
            text: 'Getting Started',
            link: '/getting-started',
          },
          {
            text: 'API Reference',
            link: '/api-reference',
          },
          {
            text: 'Configuration',
            link: '/configuration',
          },
        ],
      },
    ],
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
