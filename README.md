# Bag of Words

[Bag of Words](https://emosenkis.github.io/bag-of-words/) makes printable words you can cut out, shuffle, and arrange into stories, sentences, or Ransom Notes–style answers. Choose the word style and amount of variety you want, tune the page layout, then generate a PDF, printable HTML, Typst source, or plain-text list.

The application is deliberately built around word-frequency data, not source prose. It does not embed or send the underlying texts; generation takes place locally in the browser.

## How it works

1. The chosen embedded frequency table provides the candidate words and their relative frequencies.
2. The generator creates a broad palette, guarantees a configurable number of high-frequency words, then assigns the remaining cards according to square-root frequency weighting. This keeps function words useful without allowing a tiny handful of words to take over the deck.
3. Cards are sorted by rendered word width and then alphabetically. Each page is filled top-to-bottom in columns sized to their widest word, so similarly sized words pack together.
4. Pick a typeface from the preview cards, then set the word size, paper size, page direction, and cutting space. PDF output is compiled in-browser with the selected typeface; no fallback download is substituted if that compiler fails.

## Typefaces

The searchable picker previews 24 typefaces, including Libertinus Serif and a deliberately varied range of Google Fonts. The same selected font is fetched for PDF generation, so the PDF and printable HTML are laid out with that typeface rather than a substitute. Generating a PDF with a Google Font therefore needs an internet connection the first time that font is used.

## Optional word pieces

Choose **Include punctuation** to add sentence-building marks. Choose **Split common inflected forms** to turn only attested forms into reusable pieces such as `walk` and `-ed`; the root must also occur in the selected corpus. The implementation uses the maintained [rust-stemmers](https://crates.io/crates/rust-stemmers) Snowball implementation as a conservative check, rather than treating every algorithmic stem as a usable word.

## Included frequency data

| Choice in the app | Included file | Origin |
| --- | --- | --- |
| Asimov magazine frequency data | [assets/asimov.tsv](assets/asimov.tsv) | A cleaned, frequency-only table derived from English-language OCR in the [Internet Archive Asimov Magazine collection](https://archive.org/details/asimovmagazine). |
| General fiction | [assets/fiction-xlsx.tsv](assets/fiction-xlsx.tsv) | A frequency-only conversion of the fiction column in the [COCA word-frequency corpus](https://www.wordfrequency.info/samples.asp), supplied in `wordFrequency.xlsx`. The original workbook and its source texts are not redistributed here. |
| English Wikipedia | [assets/wikipedia.tsv](assets/wikipedia.tsv) | A frequency-only, Zipf-scaled conversion of Wiktionary's ranked list from a [one-million-sentence English Wikipedia corpus](https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists/English/Wikipedia_(2016)). Regenerate it with `python3 scripts/build_wikipedia_corpus.py`. |

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
