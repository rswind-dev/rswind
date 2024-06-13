import init, { createGenerator } from './binding/binding_core_wasm.js'
import type { Generator, GeneratorConfig } from './binding/binding_core_wasm'
import wasm from './binding/binding_core_wasm_bg.wasm'

export function extract(): Set<string> {
  const set = new Set<string>()
  const nodeList = document.body.querySelectorAll('*')
  for (const node of nodeList) {
    for (const className of node.classList) {
      set.add(className)
    }
  }
  return set
}

interface HtmlGenerator {
  inner: Generator
  observer: MutationObserver | null
  styleElem: HTMLStyleElement | null
  watch: () => void
}

function getStyleElement(): HTMLStyleElement {
  const elem = document.getElementById('__arrow_style__')
  if (elem) {
    if (!(elem instanceof HTMLStyleElement)) {
      throw new TypeError('style element not found')
    }
    return elem
  }

  const style = document.createElement('style')
  style.id = '__arrow_style__'
  document.head.appendChild(style)
  return style
}

export async function createHtmlGenerator(config?: GeneratorConfig): Promise<HtmlGenerator> {
  await init(wasm)
  const inner = createGenerator(config)
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === 'class' && mutation.target instanceof Element) {
        updateCss([...mutation.target.classList])
      }
    })
  })
  const styleElem = getStyleElement()

  function updateCss(candidates: string[] = [...extract()]) {
    const css = inner.generateWith(candidates)
    styleElem.textContent = css
  }

  return {
    inner,
    observer,
    styleElem,
    watch() {
      if (document.readyState !== 'complete') {
        document.addEventListener('DOMContentLoaded', () => updateCss())
      }
      observer.observe(document.body, {
        childList: true,
        subtree: true,
        attributes: true,
      })
    },
  }
}
