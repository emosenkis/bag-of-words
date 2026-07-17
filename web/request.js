export function requestFromValues(values) {
  return {
    corpus: values.corpus,
    count: Number(values.count),
    palette_size: Number(values.paletteSize),
    required_head: Number(values.requiredHead),
    seed: Number(values.seed),
    format: values.format,
  };
}
