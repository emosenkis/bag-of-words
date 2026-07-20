import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("the generator form uses clear labels and exposes layout spacing", async () => {
  const page = await readFile(new URL("./index.html", import.meta.url), "utf8");
  const fonts = await readFile(new URL("./fonts.js", import.meta.url), "utf8");

  assert.match(page, /Bag of Words/);
  assert.match(page, /Word style/);
  assert.match(page, /How many words\?/);
  assert.match(page, /Unique words/);
  assert.match(page, /Paper size/);
  assert.match(page, /Space between rows/);
  assert.match(page, /Space between columns/);
  assert.match(page, /Pick a typeface/);
  assert.match(page, /name="fontFamily"/);
  assert.match(page, /Search typefaces/);
  assert.match(fonts, /id: "literata"/);
  assert.match(fonts, /id: "uncial-antiqua"/);
  assert.match(fonts, /id: "press-start-2p"/);
  assert.match(page, /name="rowSpacing"/);
  assert.match(page, /name="columnSpacing"/);
  assert.match(page, /PDF<\/option>/);
  assert.doesNotMatch(page, /PDF via Typst/);
  assert.match(page, /https:\/\/github\.com\/emosenkis\/bag-of-words/);
});
