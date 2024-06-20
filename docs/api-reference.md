---
outline: deep
---

# API Reference

## `rswind` package

`rswind` provide a set of utilities for you to generate css manually.

### `Generator`

create a generator instance with `createGenerator` function.

```ts
import { createGenerator } from 'rswind'

const generator = createGenerator({
  // your options
})
```

### `generate`

`generate` is a common api, which **read from filesystem** and extract candidates from `config.contents` and generate css.

notice that `config.contents` will be set a default value if you don't pass it though `createGenerator`.

Assume that you have a `./index.vue` file:

```vue
<template>
  <div class="text-sm">
    Hello, generate api!
  </div>
</template>
```
And you can generate css with `generator.generate`:

```ts
const { css } = generator.generate()
```

```css
/* generated css */
.text-sm {
  font-size: 0.875rem;
  line-height: 1.25rem;
}
```

### `generateWith`

`generateWith` allows you to pass in a inline string content

You can pass your content type as the second argument,
like `'vue'`, `'js'`, `'html'`, `'svelte'`.
which affects the way `rswind` extract candidates.

```ts
const { css } = generator.generateWith(`
  <div class="bg-blue-50 rounded">Hello, generateWith api!</div>
`, 'vue')
```

```css
/* generated css */
.bg-blue-50 {
  background-color: #eff6ff;
}
.rounded {
  border-radius: 0.25rem;
}
```

### `generateCandidates`

`generateCandidates` is a low-level api that allows you to pass in a set of **candidates** to generate css.

for example:

```ts
const { css } = generator.generateCandidates(['bg-red-500', 'text-blue-500'])
```

```css
/* generated css */
.bg-red-500 {
  background-color: #ef4444;
}
.text-blue-500 {
  color: #3b82f6;
}
```
