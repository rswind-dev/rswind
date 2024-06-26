#!/usr/bin/env -S deno run -A

// As napi-rs can't generate typescript types like
// wasm-bindgen's #[wasm_bindgen(typescript_custom_section)] does,
// we need to generate typescript types from json schema.
//
// Notice: This script should only run though `napi build --pipe`

import { bold, red } from '@std/fmt/colors'
import { compileFromFile } from 'npm:json-schema-to-typescript'

const files = Deno.args.filter(arg => arg.endsWith('.d.ts'))

if (files.length === 0) {
  Deno.exit(0)
}

const schemaPath = Deno.makeTempFileSync()

const command = new Deno.Command('cargo', {
  args: [
    'run',
    '--features',
    'json_schema',
    '--bin',
    'json_schema',
    '--color',
    'always',
  ],
  stdout: 'inherit',
  stderr: 'inherit',
  env: {
    SCHEMA_OUT_PATH: schemaPath,
  },
})

const output = await command.output()

if (!output.success) {
  Deno.stderr.write(output.stderr)
  console.error(red(bold('Something went wrong while running cargo ↑')))
  Deno.exit(output.code)
}

const types = await compileFromFile(schemaPath)

// We currently just "append" the generated types to the file
// so this script won't act exactly what we want when running multiple times
// but it's fine for now, as we only run this at though `napi build --pipe` command
files.forEach((path) => {
  Deno.writeTextFileSync(path, types, { append: true })
  console.log(`Generated types to ${bold(path)}`)
})
