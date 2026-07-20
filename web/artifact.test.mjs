import assert from "node:assert/strict";
import test from "node:test";
import { artifactFor } from "./artifact.js";

test("creates a printable HTML preview artifact", () => {
  const artifact = artifactFor("html", "<p>deck</p>");

  assert.equal(artifact.filename, "bag-of-words.html");
  assert.equal(artifact.blob.type, "text/html;charset=utf-8");
});

test("creates a PDF preview artifact", () => {
  const artifact = artifactFor("pdf", new Uint8Array([37, 80, 68, 70]));

  assert.equal(artifact.filename, "bag-of-words.pdf");
  assert.equal(artifact.blob.type, "application/pdf");
});
