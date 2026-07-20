import assert from "node:assert/strict";
import test from "node:test";
import { typstAssets, typstImportMap } from "./typst-loader.js";

test("pins browser-compatible Typst JavaScript and WASM assets", () => {
  const assets = typstAssets();

  assert.equal(assets.module, "https://unpkg.com/@myriaddreamin/typst.ts@0.7.0/dist/esm/index.mjs");
  assert.equal(assets.compiler, "https://unpkg.com/@myriaddreamin/typst-ts-web-compiler@0.7.0/pkg/wasm-pack-shim.mjs");
  assert.equal(assets.wasm, "https://unpkg.com/@myriaddreamin/typst-ts-web-compiler@0.7.0/pkg/typst_ts_web_compiler_bg.wasm");
  assert.equal(
    typstImportMap()["@myriaddreamin/typst.ts/"],
    "https://unpkg.com/@myriaddreamin/typst.ts@0.7.0/dist/esm/",
  );
});
