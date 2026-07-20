import assert from "node:assert/strict";
import test from "node:test";
import { readFile } from "node:fs/promises";

test("uses bundled Typst modules rather than runtime CDN imports", async () => {
  const source = await readFile(new URL("./typst-loader.js", import.meta.url), "utf8");

  assert.match(source, /from "@myriaddreamin\/typst\.ts"/);
  assert.match(source, /module_or_path: compilerWasm/);
  assert.doesNotMatch(source, /https:\/\/(unpkg|esm\.sh)/);
});
