const formats = {
  html: { filename: "bag-of-words.html", mime: "text/html;charset=utf-8" },
  pdf: { filename: "bag-of-words.pdf", mime: "application/pdf" },
  txt: { filename: "bag-of-words.txt", mime: "text/plain;charset=utf-8" },
  typst: { filename: "bag-of-words.typ", mime: "text/plain;charset=utf-8" },
};

export function artifactFor(format, content) {
  const definition = formats[format];
  if (!definition) throw new Error(`Unsupported preview format: ${format}`);
  return {
    filename: definition.filename,
    blob: new Blob([content], { type: definition.mime }),
  };
}
