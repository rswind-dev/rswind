import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  target: 'node16',
  inject: ["src/esm-shims.ts"],
  clean: true,
  dts: true,
  shims: true,
});
