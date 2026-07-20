const formats = {
  html: { filename: "word-deck.html", mime: "text/html;charset=utf-8" },
  pdf: { filename: "word-deck.pdf", mime: "application/pdf" },
  txt: { filename: "word-deck.txt", mime: "text/plain;charset=utf-8" },
  typst: { filename: "word-deck.typ", mime: "text/plain;charset=utf-8" },
};

export function artifactFor(format, content) {
  const definition = formats[format];
  if (!definition) throw new Error(`Unsupported preview format: ${format}`);
  return {
    filename: definition.filename,
    blob: new Blob([content], { type: definition.mime }),
  };
}
