import assert from "node:assert/strict";
import test from "node:test";
import { requestFromValues } from "./request.js";

test("builds a typed generation request from form values", () => {
  assert.deepEqual(
    requestFromValues({
      corpus: "asimov",
      count: "2800",
      paletteSize: "700",
      requiredHead: "200",
      seed: "20260717",
      fontSize: "16",
      fontFamily: "space-grotesk",
      rowSpacing: "3",
      columnSpacing: "4",
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
      font_family: "space-grotesk",
      row_spacing: 3,
      column_spacing: 4,
      paper_size: "a4",
      orientation: "landscape",
      format: "typst",
    },
  );
});
