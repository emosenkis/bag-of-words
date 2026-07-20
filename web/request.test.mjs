import assert from "node:assert/strict";
import test from "node:test";
import { requestFromValues } from "./request.js";

test("builds a typed generation request from form values", () => {
  assert.deepEqual(
    requestFromValues({
      corpus: "asimov",
      count: "2800",
      paletteSize: "700",
      includeCommon: "on",
      requiredHead: "200",
      seed: "20260717",
      fontSize: "16",
      fontFamily: "space-grotesk",
      rowSpacing: "3",
      columnSpacing: "4",
      includePunctuation: "on",
      includeStems: "on",
      paperSize: "a4",
      orientation: "landscape",
      format: "typst",
    }),
    {
      corpus: "asimov",
      count: 2800,
      palette_size: 700,
      required_head: 200,
      seed: 20260717,
      font_size: 16,
      font_family: "Space Grotesk",
      font_url: "https://fonts.gstatic.com/s/spacegrotesk/v22/V8mQoQDjQSkFtoMM3T6r8E7mF71Q-gOoraIAEj4PVksj.ttf",
      row_spacing: 3,
      column_spacing: 4,
      include_punctuation: true,
      include_stems: true,
      paper_size: "a4",
      orientation: "landscape",
      format: "typst",
    },
  );
});
