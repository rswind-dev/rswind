import path from 'node:path'
import { describe, expect, it } from 'vitest'
import { createGenerator } from '../src/index'

describe('rswind', () => {
  it('should work', () => {
    const app = createGenerator()

    const res = app.generateWith([
      [
        path.resolve(process.cwd(), 'src/index.html'),
        '<div class="text-blue-500">Hello World</div>',
      ],
    ])

    expect(res.css).toMatchInlineSnapshot(`
      ".text-blue-500 {
        color: #3b82f6;
      }
      "
    `)
  })

  it('should work with custom utilities', () => {
    const app = createGenerator({
      config: {
        staticUtilities: {
          aa: {
            color: 'red',
          },
        },
        utilities: [
          {
            key: 'foo',
            css: {
              color: '$0:color',
            },
            modifier: {
              type: 'number',
              theme: 'opacity',
            },
            theme: 'colors',
            type: 'color',
          },
        ],
      },
    })

    const res = app.generateString('aa foo-red-500 foo-[#123456] foo-[12px]', 'unknown')

    expect(res.css).toMatchInlineSnapshot(`
      ".aa {
        color: red;
      }
      .foo-\\[\\#123456\\] {
        color: #123456;
      }
      .foo-red-500 {
        color: #ef4444;
      }
      "
    `)
  })

  it('should run with array of candidate', () => {
    const app = createGenerator()

    const res = app.generateCandidate([
      'text-blue-500',
      'flex',
    ])

    expect(res.css).toMatchInlineSnapshot(`
      ".flex {
        display: flex;
      }
      .text-blue-500 {
        color: #3b82f6;
      }
      "
    `)
  })
})
