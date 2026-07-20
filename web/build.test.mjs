import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("build configuration bundles the browser application for the Pages subpath", async () => {
  const manifest = JSON.parse(await readFile(new URL("../package.json", import.meta.url)));
  const viteConfig = await readFile(new URL("../vite.config.js", import.meta.url), "utf8");

  assert.equal(manifest.scripts.build, "vite build");
  assert.equal(manifest.dependencies["@myriaddreamin/typst.ts"], "0.7.0");
  assert.match(viteConfig, /base:\s*"\/bag-of-words\/"/);
  assert.match(viteConfig, /outDir:\s*"dist"/);
});
