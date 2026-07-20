import { fontFor } from "./fonts.js";

export function requestFromValues(values) {
  const font = fontFor(values.fontFamily);
  return {
    corpus: values.corpus,
    count: Number(values.count),
    palette_size: Number(values.paletteSize),
    required_head: values.includeCommon === "on" ? Number(values.requiredHead) : 0,
    seed: Number(values.seed),
    font_size: Number(values.fontSize),
    font_family: font.fontFamily,
    font_url: font.fontUrl,
    row_spacing: Number(values.rowSpacing),
    column_spacing: Number(values.columnSpacing),
    include_punctuation: values.includePunctuation === "on",
    include_stems: values.includeStems === "on",
    paper_size: values.paperSize,
    orientation: values.orientation,
    format: values.format,
  };
}
