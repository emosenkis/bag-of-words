const assets = Object.freeze({
  module: "https://unpkg.com/@myriaddreamin/typst.ts@0.7.0/dist/esm/index.mjs",
  compiler: "https://unpkg.com/@myriaddreamin/typst-ts-web-compiler@0.7.0/pkg/wasm-pack-shim.mjs",
  wasm: "https://unpkg.com/@myriaddreamin/typst-ts-web-compiler@0.7.0/pkg/typst_ts_web_compiler_bg.wasm",
});

export function typstAssets() {
  return assets;
}

export async function compileWithTypst(source) {
  const { $typst } = await import(assets.module);
  $typst.setCompilerInitOptions({
    getWrapper: () => import(assets.compiler),
    getModule: () => assets.wasm,
  });
  return $typst.pdf({ mainContent: source });
}
