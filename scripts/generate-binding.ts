#!/usr/bin/env -S deno run -A

import { sortBy } from '@std/collections'
import { bold, red } from '@std/fmt/colors'
import { existsSync } from '@std/fs'

enum TypeDefKind {
  Const = 'const',
  Enum = 'enum',
  StringEnum = 'string_enum',
  Interface = 'interface',
  Fn = 'fn',
  Struct = 'struct',
  Impl = 'impl',
}

interface TypeDefLine {
  kind: TypeDefKind
  name: string
  original_name?: string
  def: string
  js_doc?: string
  js_mod?: string
}

const TOP_LEVEL_NAMESPACE = '__TOP_LEVEL_MODULE__'

async function readIntermediateTypeFile(file: string) {
  const content = await Deno.readTextFile(file)
  const defs = content
    .split('\n')
    .filter(Boolean)
    .map((line) => {
      line = line.trim()
      if (!line.startsWith('{')) {
        // crateName:{ "def": "", ... }
        const start = line.indexOf(':') + 1
        line = line.slice(start)
      }
      return JSON.parse(line) as TypeDefLine
    })

  // move all `struct` def to the very top
  // and order the rest alphabetically.
  return defs.sort((a, b) => {
    if (a.kind === TypeDefKind.Struct) {
      if (b.kind === TypeDefKind.Struct) {
        return a.name.localeCompare(b.name)
      }
      return -1
    }
    else if (b.kind === TypeDefKind.Struct) {
      return 1
    }
    else {
      return a.name.localeCompare(b.name)
    }
  })
}

function preprocessTypeDef(defs: TypeDefLine[]): Map<string, TypeDefLine[]> {
  const namespaceGrouped = new Map<string, TypeDefLine[]>()
  const classDefs = new Map<string, TypeDefLine>()

  for (const def of defs) {
    const namespace = def.js_mod ?? TOP_LEVEL_NAMESPACE
    if (!namespaceGrouped.has(namespace)) {
      namespaceGrouped.set(namespace, [])
    }

    const group = namespaceGrouped.get(namespace)!

    if (def.kind === TypeDefKind.Struct) {
      group.push(def)
      classDefs.set(def.name, def)
    }
    else if (def.kind === TypeDefKind.Impl) {
      // merge `impl` into class definition
      const classDef = classDefs.get(def.name)
      if (classDef) {
        if (classDef.def) {
          classDef.def += '\n'
        }

        classDef.def += def.def
      }
    }
    else {
      group.push(def)
    }
  }

  return namespaceGrouped
}

export async function processTypeDef(
  intermediateTypeFile: string,
) {
  const exports: string[] = []
  const defs = await readIntermediateTypeFile(intermediateTypeFile)
  const groupedDefs = preprocessTypeDef(defs)

  sortBy(Array.from(groupedDefs), ([namespace]) => namespace).forEach(
    ([namespace, defs]) => {
      if (namespace === TOP_LEVEL_NAMESPACE) {
        for (const def of defs) {
          switch (def.kind) {
            case TypeDefKind.Const:
            case TypeDefKind.Enum:
            case TypeDefKind.StringEnum:
            case TypeDefKind.Fn:
            case TypeDefKind.Struct: {
              exports.push(def.name)
              if (def.original_name && def.original_name !== def.name) {
                exports.push(def.original_name)
              }
              break
            }
            default:
              break
          }
        }
      }
      else {
        exports.push(namespace)
      }
    },
  )
  return exports
}

export async function generateBinding() {
  const tempFilePath = await Deno.makeTempFile()

  const generateTypeCommand = new Deno.Command('cargo', {
    args: [
      'check',
      '-p',
      'binding_core_node',
      '--color',
      'always',
    ],
    stdout: 'inherit',
    stderr: 'inherit',
    env: {
      TYPE_DEF_TMP_PATH: tempFilePath,
    },
  })

  const generateTypeOutput = await generateTypeCommand.output()

  if (!generateTypeOutput.success) {
    Deno.stderr.write(generateTypeOutput.stderr)
    console.error(red(bold('Something went wrong while generating types ↑')))
    Deno.exit(generateTypeOutput.code)
  }

  const idents = await processTypeDef(tempFilePath)

  const exports = idents.map(ident => `export const ${ident} = binding.${ident}`).join('\n')

  return exports
}

if (import.meta.main) {
  console.log('Generating binding...')
  console.log(Deno.args[0])
  if (!Deno.args[0] ||!existsSync(Deno.args[0])) {
    console.error(red(bold('Please provide a valid path to write the generated binding')))
    Deno.exit(1)
  }
  const exports = await generateBinding()
  Deno.writeFileSync(Deno.args[0], new TextEncoder().encode(exports))
}
