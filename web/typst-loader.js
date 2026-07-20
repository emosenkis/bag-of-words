import { $typst } from "@myriaddreamin/typst.ts";
import * as compilerWrapper from "@myriaddreamin/typst-ts-web-compiler";
import compilerWasm from "@myriaddreamin/typst-ts-web-compiler/wasm?url";

let compilerConfigured = false;

export async function compileWithTypst(source) {
  if (document.readyState !== "complete") {
    await new Promise((resolve) => window.addEventListener("load", resolve, { once: true }));
  }
  await document.fonts.ready;
  if (!compilerConfigured) {
    $typst.setCompilerInitOptions({
      getWrapper: () => Promise.resolve(compilerWrapper),
      getModule: () => ({ module_or_path: compilerWasm }),
    });
    compilerConfigured = true;
  }
  return $typst.pdf({ mainContent: source });
}
