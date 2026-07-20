import assert from "node:assert/strict";
import test from "node:test";
import { compilePdf } from "./pdf.js";

test("reports a PDF compiler failure without creating a download", async () => {
  const result = await compilePdf(
    "#set text[words]",
    async () => {
      throw new Error("compiler unavailable");
    },
  );

  assert.equal(result.ok, false);
  assert.match(result.message, /compiler unavailable/);
});

test("returns the generated PDF for preview and later download", async () => {
  const pdf = new Uint8Array([37, 80, 68, 70]);
  const result = await compilePdf("#set text[words]", async () => pdf);

  assert.equal(result.ok, true);
  assert.equal(result.content, pdf);
});
