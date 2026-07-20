import { $typst } from "@myriaddreamin/typst.ts";
import * as compilerWrapper from "@myriaddreamin/typst-ts-web-compiler";
import compilerWasm from "@myriaddreamin/typst-ts-web-compiler/wasm?url";

let compilerConfigured = false;
const loadedFonts = new Set();

async function loadFont(font) {
  if (loadedFonts.has(font.id)) return;
  const response = await fetch(font.fontUrl);
  if (!response.ok) throw new Error(`Could not load ${font.label} for PDF output.`);
  const bytes = new Uint8Array(await response.arrayBuffer());
  const fontBuilder = await $typst.getFontResolver();
  const info = await fontBuilder.getFontInfo(bytes);
  await $typst.setFonts([{ info, blob: () => bytes }]);
  loadedFonts.add(font.id);
}

export async function compileWithTypst(source, font) {
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
  await loadFont(font);
  return $typst.pdf({ mainContent: source });
}
