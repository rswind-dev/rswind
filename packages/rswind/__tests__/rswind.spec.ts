import path from "path";
import { createApp } from "../bindings/index.js";
import { describe, expect, it } from "vitest";

describe("rswind", () => {
  it("should work", () => {
    const app = createApp();

    const res = app.generateWith([
      [
        path.resolve(process.cwd(), "src/index.html"),
        '<div class="text-blue-500">Hello World</div>',
      ],
    ]);

    expect(res).toMatchInlineSnapshot(`
      ".text-blue-500 {
        color: #3b82f6;
      }
      "
    `);
  });

  it("should work with custom utilities", () => {
    const app = createApp({
      config: {
        staticUtilities: {
          "aa": {
            "color": "red",
          },
        },
        utilities: [
          {
            key: "foo",
            css: {
              "color": "$0:color",
            },
            modifier: {
              type: "raw",
              value: "50",
            },
            theme: "colors",
            type: "color",
          },
        ],
      },
    });

    const res = app.generateString("aa foo-red-500 foo-[#123456] foo-[12px]", "unknown");

    expect(res).toMatchInlineSnapshot(`
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
  });

  it("should run with array of candidate", () => {
    const app = createApp();

    const res = app.generateCandidate([
      "text-blue-500",
      "flex",
    ]);

    expect(res).toMatchInlineSnapshot(`
      ".flex {
        display: flex;
      }
      .text-blue-500 {
        color: #3b82f6;
      }
      "
    `);
  })
});
