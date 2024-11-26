---
outline: deep
---

# Wasm Runtime

Rswind can run with only a html file

## Usage
Add script down below to add rswind to your existing project

```html
<script src="https://esm.sh/@rswind/wasm-runtime" type="module"></script>
```

to your html entry file, styles will be apply instantly.

## Configuration

To add configurations, use `createHtmlGenerator` to do so.

```html
<script type="module">
import { createHtmlGenerator } from 'https://esm.sh/@rswind/wasm-runtime/api'

const generator = await createHtmlGenerator({
  theme: {
    extend: {
      colors: {
        primary: "#3490dc",
        secondary: "#ffed4a",
        danger: "#e3342f"
      }
    },
  },
})

generator.watch()
</script>
```
