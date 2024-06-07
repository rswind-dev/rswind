import init, { generate, generateWith } from './binding/binding_core_wasm.js'
import wasm from './binding/binding_core_wasm_bg.wasm'

function extract(): Set<string> {
  const set = new Set<string>()
  const nodeList = document.body.querySelectorAll('*')
  for (const node of nodeList) {
    for (const className of node.classList) {
      set.add(className)
    }
  }
  return set
}

// @ts-expect-error: native option is not yet implemented
function _generateNative(): string {
  return generate(document.body.innerHTML, 'html')
}

let observer: MutationObserver | null = null
let styleElem: HTMLStyleElement | null = null

function getStyleElement(): HTMLStyleElement {
  if (styleElem) {
    return styleElem
  }

  const style = document.createElement('style')
  style.id = '__arrow_style__'
  document.head.appendChild(style)
  styleElem = style
  return style
}

function updateCss(candidates: string[] = [...extract()]) {
  const css = generateWith(candidates)
  const element = getStyleElement()

  element.textContent = css
}

async function main() {
  if (document.readyState !== 'complete') {
    document.addEventListener('DOMContentLoaded', main)
    return
  }

  updateCss()

  // MutationObserver
  if (!observer) {
    observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.target instanceof Element) {
          updateCss([...mutation.target.classList])
        }
      })
    })

    observer.observe(document.body, {
      childList: true,
      subtree: true,
      attributes: true,
    })
  }
}

init(wasm).then(() => {
  main()
})
