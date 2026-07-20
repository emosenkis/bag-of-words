import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("the generator form uses clear labels and exposes layout spacing", async () => {
  const page = await readFile(new URL("./index.html", import.meta.url), "utf8");

  assert.match(page, /Word frequency source/);
  assert.match(page, /Total word cards/);
  assert.match(page, /Paper size/);
  assert.match(page, /Row spacing/);
  assert.match(page, /Column spacing/);
  assert.match(page, /name="rowSpacing"/);
  assert.match(page, /name="columnSpacing"/);
  assert.match(page, /PDF<\/option>/);
  assert.doesNotMatch(page, /PDF via Typst/);
  assert.match(page, /https:\/\/github\.com\/emosenkis\/bag-of-words/);
});
