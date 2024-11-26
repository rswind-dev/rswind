---
outline: deep
---

# Command Line Interface

If you want to use `rswind` in your project without a framework, you can use `@rswind/cli`.

## Install

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

## Usage

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
