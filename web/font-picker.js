import { FONT_CHOICES } from "./fonts.js";

const picker = document.querySelector("[data-font-picker]");
const corpus = document.querySelector('select[name="corpus"]');
const corpusCredit = document.querySelector("#corpus-credit");

const corpusCredits = {
  asimov: "From 90 issues of Asimov's Science Fiction, preserved by the Internet Archive.",
  "fiction-xlsx": "From the fiction section of the supplied word-frequency workbook.",
  wikipedia: "From a ranked list built from one million English Wikipedia sentences.",
};

if (corpus && corpusCredit) {
  corpus.addEventListener("change", () => { corpusCredit.textContent = corpusCredits[corpus.value]; });
}

if (picker) {
  const fontFaces = document.createElement("style");
  fontFaces.textContent = FONT_CHOICES.map((font) => `@font-face { font-family: "${font.fontFamily}"; src: url("${font.fontUrl}") format("truetype"); font-display: swap; }`).join("\n");
  document.head.append(fontFaces);
  const input = picker.querySelector('input[name="fontFamily"]');
  const search = picker.querySelector(".font-search");
  const options = picker.querySelector(".font-options");

  for (const font of FONT_CHOICES) {
    const choice = document.createElement("button");
    choice.type = "button";
    choice.className = "font-choice";
    choice.dataset.font = font.id;
    choice.style.setProperty("--preview-font", `"${font.fontFamily}"`);
    choice.innerHTML = `<span>${font.label}</span><span class="font-sample">Make a story</span>`;
    options.append(choice);
  }
  const choices = options.querySelectorAll("[data-font]");

  function selectFont(id) {
    input.value = id;
    for (const choice of choices) {
      const selected = choice.dataset.font === id;
      choice.setAttribute("aria-pressed", String(selected));
    }
  }

  for (const choice of choices) {
    choice.addEventListener("click", () => selectFont(choice.dataset.font));
  }

  search.addEventListener("input", () => {
    const query = search.value.trim().toLocaleLowerCase();
    for (const choice of choices) {
      choice.hidden = !choice.textContent.toLocaleLowerCase().includes(query);
    }
  });
}
