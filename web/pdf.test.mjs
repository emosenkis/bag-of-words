import assert from "node:assert/strict";
import test from "node:test";
import { compilePdf } from "./pdf.js";

test("reports a PDF compiler failure without downloading Typst source", async () => {
  let downloads = 0;

  const result = await compilePdf(
    "#set text[words]",
    async () => {
      throw new Error("compiler unavailable");
    },
    () => {
      downloads += 1;
    },
  );

  assert.equal(result.ok, false);
  assert.match(result.message, /compiler unavailable/);
  assert.equal(downloads, 0);
});
