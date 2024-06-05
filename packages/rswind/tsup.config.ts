import { defineConfig } from "tsup";

export default defineConfig({
  entry: {
    index: "src/index.ts",
  },
  format: ["esm", "cjs"],
  target: "es2015",
  loader: { ".node": "file" },
  external: [
      // ignore .node
    /.*\.node$/,
  ],
  clean: true,
  dts: true,
});
