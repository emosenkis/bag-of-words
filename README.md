# Word Deck WASM

Browser-run Rust/WASM word-deck generator. It embeds two frequency-only corpora:

- `assets/asimov.tsv`: the cleaned Asimov magazine frequency table;
- `assets/fiction-xlsx.tsv`: the fiction-column frequency table.

No source prose is embedded in this repository.

## Build

```bash
npm ci
cargo test
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/release/word_deck.wasm
npm run build
python3 -m http.server --directory web/dist 8000
```

Open `http://localhost:8000`. Choose the corpus, deck parameters, paper size (Letter, A3, A4, or A5), orientation, and an export. TXT, HTML, and Typst source download directly. The HTML export measures the selected browser font and packs sorted cards into full-height, width-fitting columns. PDF uses the Typst.ts browser/WASM compiler; a compiler failure is reported without downloading a different format.

## Browser smoke test

1. Generate a seeded TXT deck and verify it contains the requested number of lines.
2. Generate HTML and open/print it; cards must be sorted by measured width and packed into 1.5-line-spaced, full-height columns for the selected paper.
3. Generate Typst and compile it with local Typst if desired.
4. Generate PDF with network access; confirm a PDF downloads. If Typst.ts fails to load, confirm an error is shown and no `.typ` file downloads.
