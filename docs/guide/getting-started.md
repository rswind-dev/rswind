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

```ts{2,7}
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
