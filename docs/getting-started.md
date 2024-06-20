---
outline: deep
---

# Getting Started

## Framework Integration

In most cases, you'll install `rswind` as a framework plugin.

### Requirements

- [Node.js](https://nodejs.org/) version 18 or higher.

### Vite

::: code-group

```bash [npm]
npm install @rswind/vite -D
```

```bash [yarn]
yarn add @rswind/vite -D
```

```bash [pnpm]
pnpm install @rswind/vite -D
```

:::

Then, add `@rswind/vite` to your `vite.config.ts`:

```ts
import { defineConfig } from 'vite'
import rswind from '@rswind/vite'
// other imports

export default defineConfig({
  plugins: [
    rswind(),
    // other plugins
  ]
})
```

Next, add `rswind.css` to your entry file:

::: code-group

```ts{3} [vue]
import { createApp } from 'vue'
import App from './App.vue'
import 'rswind.css'

createApp(App).mount('#app')
```

```tsx{4} [react]
import React from 'react'
import ReactDOM from 'react-dom'
import App from './App'
import 'rswind.css'

ReactDOM.render(<App />, document.getElementById('root'))
```

```ts{2} [svelte]
import App from './App.svelte'
import 'rswind.css'

new App({
  target: document.body
})
```

:::

Now, just run your project, you can start typing `rswind` classes in your components!

Have a try:

::: code-group

```vue
<template>
  <div class="bg-blue-50 text-2xl text-blur-500">
    Hello, rswind!
  </div>
</template>
```

```jsx [react]
export default () => {
  return <div className="bg-blue-50 text-2xl text-blur-500">Hello, rswind!</div>
}
```

```html [svelte]
<div class="bg-blue-50 text-2xl text-blue-500">Hello, rswind!</div>
```

:::

### Webpack

```rust
todo!("Coming soon...")
```

## Command Line Usage

If you want to use `rswind` in your project without a framework, you can use `@rswind/cli`.

### Install

::: code-group

```bash [npm]
npm install @rswind/cli
```

```bash [yarn]
yarn add @rswind/cli
```

```bash [pnpm]
pnpm install @rswind/cli
```

:::

### Usage

The CLI provides a simple way to generate CSS from your HTML files.

```
Usage: rswind [OPTIONS] [CONTENT]... [COMMAND]

Commands:
  debug
  init
  help   Print this message or the help of the given subcommand(s)

Arguments:
  [CONTENT]...

Options:
  -o <OUTPUT>            Output path (default: stdout)
  -w                     Enable watch mode
  -s, --strict           Enable strict mode
      --config <CONFIG>  Path to config file [default: rswind.config.json]
  -c, --cwd <CWD>        Path to working directory [default: .]
  -h, --help             Print help
  -V, --version          Print version
```

For example, to generate CSS from all project files in the current directory:

```bash
npx rswind -o ./css/style.css
```

By default, rswind reads all `html,js/ts(x),vue,svelte,mdx` to find files to process. You can also specify a custom glob pattern, for example, only process HTML files:

```bash
npx rswind './**/*.html' -o ./css/style.css
```

You can always combine multiple glob patterns:

e.g: only `html` files in `src` folder and `mdx` files in `pages` folder

```bash
npx rswind './src/**/*.html' './components/**/*.mdx' -o ./css/style.css
```
