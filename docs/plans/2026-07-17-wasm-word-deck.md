# WASM Word Deck Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a browser-run Rust/WASM application that embeds frequency-only corpora, samples a printable word deck, and exports HTML, plain text, Typst source, or browser-generated Typst PDF.

**Architecture:** A pure Rust core owns corpus parsing, deterministic two-stage sampling, morphology-safe filtering, and render-source generation. A `wasm-bindgen` boundary exposes a JSON request/response API to a dependency-light browser UI. The UI can download HTML/TXT/Typst directly and uses Typst.ts’s browser compiler for optional PDF export.

**Tech Stack:** Rust 2024, `wasm-bindgen`, `serde`, `serde_json`, `rand_chacha`, `wasm-bindgen-test`, browser ES modules, Typst.ts web compiler.

---

### Task 1: Establish the Rust core and deterministic frequency sampler

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `tests/sampling.rs`

**Step 1: Write failing tests** for parsing a TSV corpus, selecting a required high-frequency head, and producing the requested number of deterministic cards.

**Step 2: Run** `cargo test --test sampling` and confirm failure because the crate is absent.

**Step 3: Implement** the minimal core types and largest-remainder square-root allocation.

**Step 4: Run** `cargo test --test sampling`; confirm success.

### Task 2: Add render targets

**Files:**
- Create: `tests/rendering.rs`
- Modify: `src/lib.rs`

**Step 1: Write failing tests** for TXT escaping, standalone HTML output, and valid Typst source containing nonbreaking affix cards.

**Step 2: Run** `cargo test --test rendering`; confirm failure.

**Step 3: Implement** pure render functions.

**Step 4: Run** `cargo test --test rendering`; confirm success.

### Task 3: Embed frequency-only datasets and expose WASM API

**Files:**
- Create: `assets/asimov.tsv`
- Create: `assets/fiction-xlsx.tsv`
- Create: `tests/api.rs`
- Modify: `src/lib.rs`

**Step 1: Write failing tests** for selecting an embedded corpus and rejecting invalid settings.

**Step 2: Run** `cargo test --test api`; confirm failure.

**Step 3: Implement** embedded corpus lookup and a JSON WASM request/response API.

**Step 4: Run** all `cargo test`; confirm success.

### Task 4: Build the browser interface and PDF integration

**Files:**
- Create: `web/index.html`
- Create: `web/app.js`
- Create: `web/style.css`
- Create: `README.md`

**Step 1: Add a browser smoke test specification** in `README.md` for corpus selection, seeded generation, and each export.

**Step 2: Implement** controls for corpus, deck size, palette size, seed, adjective quota, and export format.

**Step 3: Implement** direct TXT/HTML/Typst downloads and Typst.ts browser PDF download, with Typst-source fallback if the compiler cannot load.

**Step 4: Build** `wasm32-unknown-unknown --release`, serve `web/`, and manually smoke-test all formats.

### Task 5: Verify release quality

**Files:**
- Modify: `README.md`

**Step 1: Run** `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and the WASM release build.

**Step 2: Confirm** assets contain only `word<TAB>frequency` rows and no source prose.

**Step 3: Commit** the fresh repository history with source, tests, assets, and documentation.
