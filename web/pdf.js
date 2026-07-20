export async function compilePdf(typst, compiler) {
  try {
    const pdf = await compiler(typst);
    return { ok: true, content: pdf, message: "Generated PDF with Typst WASM." };
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    return { ok: false, message: `Typst PDF compiler was unavailable: ${detail}` };
  }
}
