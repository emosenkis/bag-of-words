# Bag of Words

[Bag of Words](https://emosenkis.github.io/bag-of-words/) is a fully static, browser-run tool for making printable decks of story-building words. Pick a frequency source and deck size, tune the page layout, then generate a PDF, printable HTML, Typst source, or plain-text list.

The application is deliberately built around word-frequency data, not source prose. It does not embed or send the underlying texts; generation takes place locally in the browser.

## How it works

1. The chosen embedded frequency table provides the candidate words and their relative frequencies.
2. The generator creates a broad palette, guarantees a configurable number of high-frequency words, then assigns the remaining cards according to square-root frequency weighting. This keeps function words useful without allowing a tiny handful of words to take over the deck.
3. Cards are sorted by rendered word width and then alphabetically. Each page is filled top-to-bottom in columns sized to their widest word, so similarly sized words pack together.
4. The selected font size, paper size, orientation, row spacing, and column spacing control the layout. PDF output is compiled in-browser; no fallback download is substituted if that compiler fails.

## Included frequency data

| Choice in the app | Included file | Origin |
| --- | --- | --- |
| Asimov magazine frequency data | [assets/asimov.tsv](assets/asimov.tsv) | A cleaned, frequency-only table derived from English-language OCR in the [Internet Archive Asimov Magazine collection](https://archive.org/details/asimovmagazine). |
| Fiction frequency dataset | [assets/fiction-xlsx.tsv](assets/fiction-xlsx.tsv) | A frequency-only conversion of the locally supplied `wordFrequency.xlsx` fiction-genre worksheet. The original workbook and its source texts are not redistributed here. |

Both TSV files contain only `word` and `frequency` columns. They are compiled into the WebAssembly binary so the deployed site needs no server or corpus download.

## Tools

- [Rust](https://www.rust-lang.org/) implements deterministic sampling, validation, and source generation.
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) compiles the Rust core to browser WebAssembly.
- [Vite](https://vite.dev/) bundles the static web application for GitHub Pages.
- [Typst.ts](https://github.com/Myriad-Dreamin/typst.ts) compiles the PDF in the browser. The compiler is bundled during the site build rather than loaded from a CDN.

## Run locally

Prerequisites: a current Rust toolchain with the `wasm32-unknown-unknown` target, `wasm-bindgen-cli`, and Node.js with npm.

```bash
npm ci
cargo test
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/release/word_deck.wasm
npm run build
python3 -m http.server --directory web/dist 8000
```

Open `http://localhost:8000`. Generate first to review the result in the page, then download the generated file. The Download button stays disabled until generation succeeds.

## Verify changes

```bash
node --test web/*.test.mjs
cargo test
npm run build
```
