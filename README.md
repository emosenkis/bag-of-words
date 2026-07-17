# Word Deck WASM

Browser-run Rust/WASM word-deck generator. It embeds two frequency-only corpora:

- `assets/asimov.tsv`: the cleaned Asimov magazine frequency table;
- `assets/fiction-xlsx.tsv`: the fiction-column frequency table.

No source prose is embedded in this repository.

## Build

```bash
cargo test
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/release/word_deck.wasm
python3 -m http.server --directory web 8000
```

Open `http://localhost:8000`. Choose the corpus, deck parameters, and an export. TXT, HTML, and Typst source download directly. PDF uses the Typst.ts browser/WASM compiler and falls back to Typst source if it cannot load.

## Browser smoke test

1. Generate a seeded TXT deck and verify it contains the requested number of lines.
2. Generate HTML and open/print it; cards must use nonbreaking text and 1.5 line spacing.
3. Generate Typst and compile it with local Typst if desired.
4. Generate PDF with network access; if Typst.ts fails to load, confirm the `.typ` fallback downloads.
