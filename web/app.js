import init, { generate } from "./pkg/word_deck.js";
import { compilePdf } from "./pdf.js";
import { artifactFor } from "./artifact.js";
import { requestFromValues } from "./request.js";
import { compileWithTypst } from "./typst-loader.js";
import { fontFor } from "./fonts.js";

const form = document.querySelector("#generator");
const status = document.querySelector("#status");
const generateButton = document.querySelector("#generate");
const downloadButton = document.querySelector("#download");
const resultFrame = document.querySelector("#result");
let artifact;
let previewUrl;

function download(blob, filename) {
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = filename;
  link.click();
  setTimeout(() => URL.revokeObjectURL(link.href), 0);
}

async function exportPdf(typst, font) {
  return compilePdf(typst, (source) => compileWithTypst(source, font));
}

function showPreview(nextArtifact) {
  if (previewUrl) URL.revokeObjectURL(previewUrl);
  previewUrl = URL.createObjectURL(nextArtifact.blob);
  resultFrame.src = previewUrl;
  resultFrame.hidden = false;
}

downloadButton.addEventListener("click", () => {
  if (artifact) download(artifact.blob, artifact.filename);
});

await init();
status.textContent = "Ready.";

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  const values = Object.fromEntries(new FormData(form));
  const request = requestFromValues(values);
  try {
    status.textContent = "Generating…";
    generateButton.disabled = true;
    downloadButton.disabled = true;
    resultFrame.hidden = true;
    if (request.format === "pdf") {
      request.format = "typst";
      const response = JSON.parse(generate(JSON.stringify(request)));
      const result = await exportPdf(response.content, fontFor(values.fontFamily));
      if (!result.ok) throw new Error(result.message);
      artifact = artifactFor("pdf", result.content);
    } else {
      const response = JSON.parse(generate(JSON.stringify(request)));
      artifact = artifactFor(response.format, response.content);
    }
    showPreview(artifact);
    downloadButton.disabled = false;
    status.textContent = `Your bag is ready: ${artifact.filename}. Preview shown below.`;
  } catch (error) {
    artifact = undefined;
    status.textContent = `Could not generate your bag: ${error.message}`;
  } finally {
    generateButton.disabled = false;
  }
});
