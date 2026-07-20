export async function compilePdf(typst, compiler) {
  try {
    const pdf = await compiler(typst);
    return { ok: true, content: pdf, message: "Generated PDF with Typst WASM." };
  } catch (error) {
    return { ok: false, message: `Typst PDF compiler was unavailable: ${error.message}` };
  }
}
