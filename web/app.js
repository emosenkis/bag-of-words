import init, { generate } from "./pkg/word_deck.js";
import { compilePdf } from "./pdf.js";
import { requestFromValues } from "./request.js";

const form = document.querySelector("#generator");
const status = document.querySelector("#status");

function download(content, filename, mime) {
  const link = document.createElement("a");
  link.href = URL.createObjectURL(new Blob([content], { type: mime }));
  link.download = filename;
  link.click();
  setTimeout(() => URL.revokeObjectURL(link.href), 0);
}

async function exportPdf(typst) {
  return compilePdf(typst, async (source) => {
    const { $typst } = await import("https://esm.sh/@myriaddreamin/typst.ts@0.7.0?conditions=browser");
    const pdf = await $typst.pdf({ mainContent: source });
    return pdf;
  }, download);
}

await init();
status.textContent = "Ready.";

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  const values = Object.fromEntries(new FormData(form));
  const request = requestFromValues(values);
  try {
    status.textContent = "Generating…";
    if (request.format === "pdf") {
      request.format = "typst";
      const response = JSON.parse(generate(JSON.stringify(request)));
      const result = await exportPdf(response.content);
      status.textContent = result.message;
      return;
    }
    const response = JSON.parse(generate(JSON.stringify(request)));
    const mime = response.format === "html" ? "text/html;charset=utf-8" : "text/plain;charset=utf-8";
    download(response.content, `word-deck.${response.format}`, mime);
    status.textContent = `Downloaded ${response.cards.length} cards as ${response.format.toUpperCase()}.`;
  } catch (error) {
    status.textContent = `Could not generate deck: ${error.message}`;
  }
});
