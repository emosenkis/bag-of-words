export function requestFromValues(values) {
  return {
    corpus: values.corpus,
    count: Number(values.count),
    palette_size: Number(values.paletteSize),
    required_head: Number(values.requiredHead),
    seed: Number(values.seed),
    font_size: Number(values.fontSize),
    row_spacing: Number(values.rowSpacing),
    column_spacing: Number(values.columnSpacing),
    paper_size: values.paperSize,
    orientation: values.orientation,
    format: values.format,
  };
}
