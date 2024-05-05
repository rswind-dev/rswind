import init, { generate } from "../pkg/binding_core_wasm.js";
import wasm from '../pkg/binding_core_wasm_bg.wasm';

// @ts-ignore
function _extract() {
  const set = new Set();
  const nodeList = document.body.querySelectorAll("*");
  for (const node of nodeList) {
    for (const className of node.classList) {
      set.add(className);
    }
  }
  return set;
}

let observer: MutationObserver | null = null;

async function main() {
  if (document.readyState !== "complete") {
      document.addEventListener("DOMContentLoaded", main);
      return;
  }
  let html = document.body.innerHTML;
  let css = generate(html, "html");
  let style = document.createElement("style");
  style.id = "__arrow_style__";
  style.textContent = css;
  document.head.appendChild(style);

  // MutationObserver
  if (!observer) {
    observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        console.log(mutation);
        let html = document.body.innerHTML;
        let css = generate(html, "html");
        let style = document.getElementById("__arrow_style__")!;
        style.textContent = css;
      });
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true,
      attributes: true,
    });
  }
}

init(wasm).then(() => {
  main();
})
