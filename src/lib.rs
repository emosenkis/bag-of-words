#![forbid(unsafe_code)]

use std::collections::HashMap;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Corpus {
    frequencies: HashMap<String, u64>,
}

impl Corpus {
    pub fn from_tsv(input: &str) -> Result<Self, String> {
        let mut frequencies = HashMap::new();
        for (line_number, line) in input.lines().enumerate() {
            if line_number == 0 && line == "word\tfrequency" {
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }
            let (word, raw_frequency) = line
                .split_once('\t')
                .ok_or_else(|| format!("line {} is not tab-separated", line_number + 1))?;
            let frequency = raw_frequency
                .parse::<u64>()
                .map_err(|_| format!("line {} has an invalid frequency", line_number + 1))?;
            if word.is_empty() || frequency == 0 {
                return Err(format!(
                    "line {} has an empty word or zero frequency",
                    line_number + 1
                ));
            }
            frequencies.insert(word.to_owned(), frequency);
        }
        if frequencies.is_empty() {
            return Err("corpus contains no frequency rows".to_owned());
        }
        Ok(Self { frequencies })
    }

    pub fn frequency(&self, word: &str) -> Option<u64> {
        self.frequencies.get(word).copied()
    }

    fn ranked_words(&self) -> Vec<(&str, u64)> {
        let mut rows: Vec<_> = self
            .frequencies
            .iter()
            .map(|(word, frequency)| (word.as_str(), *frequency))
            .collect();
        rows.sort_unstable_by(
            |(left_word, left_frequency), (right_word, right_frequency)| {
                right_frequency
                    .cmp(left_frequency)
                    .then_with(|| left_word.cmp(right_word))
            },
        );
        rows
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeckConfig {
    pub count: usize,
    pub palette_size: usize,
    pub required_head: usize,
    pub seed: u64,
}

#[derive(Debug, Deserialize)]
struct GenerationRequest {
    corpus: String,
    count: usize,
    palette_size: usize,
    required_head: usize,
    seed: u64,
    #[serde(default = "default_font_size")]
    font_size: f64,
    #[serde(default = "default_font_family")]
    font_family: String,
    #[serde(default = "default_row_spacing")]
    row_spacing: f64,
    #[serde(default = "default_column_spacing")]
    column_spacing: f64,
    #[serde(default = "default_paper_size")]
    paper_size: String,
    #[serde(default = "default_orientation")]
    orientation: String,
    format: String,
}

fn default_paper_size() -> String {
    "letter".to_owned()
}

fn default_orientation() -> String {
    "portrait".to_owned()
}

fn default_font_size() -> f64 {
    14.0
}

fn default_font_family() -> String {
    "libertinus".to_owned()
}

fn default_row_spacing() -> f64 {
    2.4
}

fn default_column_spacing() -> f64 {
    3.0
}

#[derive(Debug, Serialize)]
pub struct GenerationResponse {
    pub format: String,
    pub cards: Vec<String>,
    pub content: String,
}

pub fn embedded_corpus(name: &str) -> Result<Corpus, String> {
    match name {
        "asimov" => Corpus::from_tsv(include_str!("../assets/asimov.tsv")),
        "fiction-xlsx" => Corpus::from_tsv(include_str!("../assets/fiction-xlsx.tsv")),
        _ => Err(format!("unknown corpus: {name}")),
    }
}

pub fn generate_json(request_json: &str) -> Result<GenerationResponse, String> {
    let request: GenerationRequest = serde_json::from_str(request_json)
        .map_err(|error| format!("invalid request JSON: {error}"))?;
    validate_font_size(request.font_size)?;
    let font = font_spec(&request.font_family)?;
    validate_spacing("row_spacing", request.row_spacing)?;
    validate_spacing("column_spacing", request.column_spacing)?;
    let corpus = embedded_corpus(&request.corpus)?;
    let cards = generate_deck(
        &corpus,
        &DeckConfig {
            count: request.count,
            palette_size: request.palette_size,
            required_head: request.required_head,
            seed: request.seed,
        },
    )?;
    let content = match request.format.as_str() {
        "txt" => render_text(&cards),
        "html" => render_html_for_page_with_typography(
            &cards,
            &request.paper_size,
            &request.orientation,
            request.font_size,
            font,
            request.row_spacing,
            request.column_spacing,
        )?,
        "typst" => render_typst_for_page_with_typography(
            &cards,
            &request.paper_size,
            &request.orientation,
            request.font_size,
            font,
            request.row_spacing,
            request.column_spacing,
        )?,
        _ => return Err(format!("unknown export format: {}", request.format)),
    };
    Ok(GenerationResponse {
        format: request.format,
        cards,
        content,
    })
}

#[wasm_bindgen]
pub fn generate(request_json: &str) -> Result<String, JsValue> {
    let response = generate_json(request_json).map_err(|error| JsValue::from_str(&error))?;
    serde_json::to_string(&response).map_err(|error| JsValue::from_str(&error.to_string()))
}

pub fn generate_deck(corpus: &Corpus, config: &DeckConfig) -> Result<Vec<String>, String> {
    if config.count == 0 || config.palette_size == 0 {
        return Err("count and palette_size must be positive".to_owned());
    }
    if config.palette_size > config.count {
        return Err("palette_size cannot exceed count".to_owned());
    }
    let ranked = corpus.ranked_words();
    let palette_size = config.palette_size.min(ranked.len());
    let head_size = config.required_head.min(palette_size);
    let mut palette: Vec<_> = ranked[..head_size].to_vec();
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);
    let mut remaining: Vec<_> = ranked[head_size..]
        .iter()
        .map(|(word, frequency)| {
            (
                -rng.random::<f64>().ln() / *frequency as f64,
                *word,
                *frequency,
            )
        })
        .collect();
    remaining.sort_unstable_by(|left, right| {
        left.0.total_cmp(&right.0).then_with(|| left.1.cmp(right.1))
    });
    palette.extend(
        remaining
            .into_iter()
            .take(palette_size - head_size)
            .map(|(_, word, frequency)| (word, frequency)),
    );

    let extra_cards = config.count - palette.len();
    let weight_total: f64 = palette
        .iter()
        .map(|(_, frequency)| (*frequency as f64).sqrt())
        .sum();
    let mut allocations: Vec<_> = palette
        .iter()
        .map(|(word, frequency)| {
            let quota = extra_cards as f64 * (*frequency as f64).sqrt() / weight_total;
            (
                (*word).to_owned(),
                1 + quota.floor() as usize,
                quota.fract(),
            )
        })
        .collect();
    let allocated: usize = allocations.iter().map(|(_, count, _)| *count).sum();
    allocations.sort_unstable_by(|left, right| {
        right
            .2
            .total_cmp(&left.2)
            .then_with(|| left.0.cmp(&right.0))
    });
    for (_, count, _) in allocations.iter_mut().take(config.count - allocated) {
        *count += 1;
    }
    allocations.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    Ok(allocations
        .into_iter()
        .flat_map(|(word, count, _)| std::iter::repeat_n(word, count))
        .collect())
}

pub fn render_text(cards: &[String]) -> String {
    format!("{}\n", cards.join("\n"))
}

pub fn render_html(cards: &[String]) -> String {
    render_html_for_page(cards, "letter", "portrait").expect("default paper is valid")
}

pub fn render_html_for_page(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
) -> Result<String, String> {
    render_html_for_page_with_font(cards, paper_size, orientation, default_font_size())
}

pub fn render_html_for_page_with_font(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
    font_size: f64,
) -> Result<String, String> {
    render_html_for_page_with_layout(
        cards,
        paper_size,
        orientation,
        font_size,
        default_row_spacing(),
        default_column_spacing(),
    )
}

pub fn render_html_for_page_with_layout(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
    font_size: f64,
    row_spacing: f64,
    column_spacing: f64,
) -> Result<String, String> {
    render_html_for_page_with_typography(
        cards,
        paper_size,
        orientation,
        font_size,
        font_spec("libertinus")?,
        row_spacing,
        column_spacing,
    )
}

pub fn render_html_for_page_with_typography(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
    font_size: f64,
    font: FontSpec,
    row_spacing: f64,
    column_spacing: f64,
) -> Result<String, String> {
    validate_font_size(font_size)?;
    validate_spacing("row_spacing", row_spacing)?;
    validate_spacing("column_spacing", column_spacing)?;
    let (width_mm, height_mm, css_paper) = page_spec(paper_size, orientation)?;
    let cards_json = serde_json::to_string(cards)
        .map_err(|error| format!("could not serialize cards for HTML: {error}"))?
        .replace("</", "<\\/");
    Ok(r##"<!doctype html>
<html><head><meta charset="utf-8"><title>Word deck</title><style>
@page { size: __PAPER__ __ORIENTATION__; margin: 4mm; }
html, body { margin: 0; padding: 0; }
@font-face { font-family: "__FONT_NAME__"; src: url("__FONT_URL__"); }
body { font: __FONT_SIZE__pt "__FONT_NAME__", __FONT_FALLBACK__; }
#deck { display: block; }
.page { box-sizing: border-box; display: flex; gap: __COLUMN_SPACING__mm; overflow: hidden; page-break-after: always; break-after: page; }
.column { box-sizing: border-box; flex: none; }
.card { display: block; margin-bottom: __ROW_SPACING__mm; line-height: 1.5; text-align: center; white-space: nowrap; }
@media screen { body { background: #eee; } .page { background: white; margin: 8mm auto; } }
</style></head><body><main id="deck"></main><script>
const cards = __CARDS__;
const page = { widthMm: __WIDTH__, heightMm: __HEIGHT__, marginMm: 4, rowSpacingMm: __ROW_SPACING__, columnSpacingMm: __COLUMN_SPACING__ };
const deck = document.querySelector("#deck");

function buildLayout() {
  const canvas = document.createElement("canvas");
  const context = canvas.getContext("2d");
  if (!(context instanceof CanvasRenderingContext2D)) throw new Error("Canvas text measurement is unavailable.");
  const style = getComputedStyle(document.body);
  context.font = `${style.fontStyle} ${style.fontWeight} ${style.fontSize} ${style.fontFamily}`;
  const pxPerMm = 96 / 25.4;
  const lineHeight = parseFloat(style.fontSize) * 1.5 + page.rowSpacingMm * pxPerMm;
  const rows = Math.max(1, Math.floor(((page.heightMm - 2 * page.marginMm) * pxPerMm) / lineHeight));
  const usableWidth = (page.widthMm - 2 * page.marginMm) * pxPerMm;
  const gap = page.columnSpacingMm * pxPerMm;
  const horizontalPadding = page.columnSpacingMm * pxPerMm;
  const measured = cards.map((word) => ({ word, width: context.measureText(word).width + horizontalPadding }));
  measured.sort((left, right) => left.width - right.width || left.word.localeCompare(right.word));

  let cursor = 0;
  while (cursor < measured.length) {
    const pageElement = document.createElement("section");
    pageElement.className = "page";
    pageElement.style.width = `${page.widthMm - 2 * page.marginMm}mm`;
    pageElement.style.height = `${page.heightMm - 2 * page.marginMm}mm`;
    let usedWidth = 0;
    while (cursor < measured.length) {
      const columnCards = measured.slice(cursor, cursor + rows);
      const columnWidth = Math.max(...columnCards.map((card) => card.width));
      const nextWidth = usedWidth === 0 ? columnWidth : usedWidth + gap + columnWidth;
      if (usedWidth > 0 && nextWidth > usableWidth) break;
      const column = document.createElement("div");
      column.className = "column";
      column.style.width = `${columnWidth}px`;
      for (const card of columnCards) {
        const element = document.createElement("span");
        element.className = "card";
        element.textContent = card.word;
        column.append(element);
      }
      pageElement.append(column);
      usedWidth = nextWidth;
      cursor += columnCards.length;
    }
    deck.append(pageElement);
  }
}

document.fonts.ready.then(buildLayout).catch((error) => { deck.textContent = `Could not lay out deck: ${error.message}`; });
</script></body></html>"##
        .replace("__PAPER__", css_paper)
        .replace("__ORIENTATION__", orientation)
        .replace("__WIDTH__", &width_mm.to_string())
        .replace("__HEIGHT__", &height_mm.to_string())
        .replace("__FONT_SIZE__", &font_size.to_string())
        .replace("__FONT_NAME__", font.name)
        .replace("__FONT_URL__", font.url)
        .replace("__FONT_FALLBACK__", font.css_fallback)
        .replace("__ROW_SPACING__", &row_spacing.to_string())
        .replace("__COLUMN_SPACING__", &column_spacing.to_string())
        .replace("__CARDS__", &cards_json))
}

pub fn render_typst(cards: &[String]) -> String {
    render_typst_for_page(cards, "letter", "portrait").expect("default paper is valid")
}

pub fn render_typst_for_page(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
) -> Result<String, String> {
    render_typst_for_page_with_font(cards, paper_size, orientation, default_font_size())
}

pub fn render_typst_for_page_with_font(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
    font_size: f64,
) -> Result<String, String> {
    render_typst_for_page_with_layout(
        cards,
        paper_size,
        orientation,
        font_size,
        default_row_spacing(),
        default_column_spacing(),
    )
}

pub fn render_typst_for_page_with_layout(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
    font_size: f64,
    row_spacing: f64,
    column_spacing: f64,
) -> Result<String, String> {
    render_typst_for_page_with_typography(
        cards,
        paper_size,
        orientation,
        font_size,
        font_spec("libertinus")?,
        row_spacing,
        column_spacing,
    )
}

pub fn render_typst_for_page_with_typography(
    cards: &[String],
    paper_size: &str,
    orientation: &str,
    font_size: f64,
    font: FontSpec,
    row_spacing: f64,
    column_spacing: f64,
) -> Result<String, String> {
    validate_font_size(font_size)?;
    validate_spacing("row_spacing", row_spacing)?;
    validate_spacing("column_spacing", column_spacing)?;
    let (width_mm, height_mm, _) = page_spec(paper_size, orientation)?;
    let sampled = cards
        .iter()
        .map(|card| format!("  \"{}\"", escape_typst(card)))
        .collect::<Vec<_>>()
        .join(",\n");
    let printable_width = width_mm - 8.0;
    let printable_height = height_mm - 8.0;
    Ok(
        r##"#set page(width: __PAGE_WIDTH__mm, height: __PAGE_HEIGHT__mm, margin: 4mm)
#set text(font: "__FONT_NAME__", size: __FONT_SIZE__pt)
#set par(leading: 0.5em)
#set table(stroke: none, inset: (x: __TABLE_INSET_X__mm, y: __ROW_SPACING__mm), align: center + horizon)

#let sampled = (
__SAMPLED__,
)
#context {
  let words = sampled.sorted().sorted(key: word => measure(text(word)).width)
  let cell-inset = __COLUMN_SPACING__mm
  let printable-width = __PRINTABLE_WIDTH__mm
  let printable-height = __PRINTABLE_HEIGHT__mm
  let row-height = measure(box(inset: (y: __ROW_SPACING__mm))[Ag]).height
  let rows-per-column = calc.floor(printable-height / row-height)
  let column-width(column) = {
    let widest = 0pt
    for word in column { widest = calc.max(widest, measure(text(word)).width) }
    widest + cell-inset
  }
  let render-page(columns) = {
    let widths = columns.map(column-width)
    let cells = ()
    for row in range(rows-per-column) {
      for column in columns {
        let cell = if row < column.len() {[#box[#column.at(row)]]} else {[]}
        cells.push(cell)
      }
    }
    align(center)[#table(columns: widths, ..cells)]
  }
  let columns = ()
  for index in range(calc.ceil(words.len() / rows-per-column)) {
    let start = index * rows-per-column
    columns.push(words.slice(start, calc.min(start + rows-per-column, words.len())))
  }
  let pages = ()
  let page-columns = ()
  let used-width = 0pt
  for column in columns {
    let width = column-width(column)
    if page-columns.len() > 0 and used-width + width > printable-width {
      pages.push(render-page(page-columns))
      pages.push(pagebreak())
      page-columns = ()
      used-width = 0pt
    }
    page-columns.push(column)
    used-width += width
  }
  if page-columns.len() > 0 { pages.push(render-page(page-columns)) }
  pages.join()
}
"##
        .replace("__PAGE_WIDTH__", &width_mm.to_string())
        .replace("__PAGE_HEIGHT__", &height_mm.to_string())
        .replace("__PRINTABLE_WIDTH__", &printable_width.to_string())
        .replace("__PRINTABLE_HEIGHT__", &printable_height.to_string())
        .replace("__FONT_SIZE__", &font_size.to_string())
        .replace("__FONT_NAME__", font.name)
        .replace("__TABLE_INSET_X__", &(column_spacing / 2.0).to_string())
        .replace("__ROW_SPACING__", &row_spacing.to_string())
        .replace("__COLUMN_SPACING__", &column_spacing.to_string())
        .replace("__SAMPLED__", &sampled),
    )
}

#[derive(Clone, Copy)]
pub struct FontSpec {
    name: &'static str,
    url: &'static str,
    css_fallback: &'static str,
}

fn font_spec(id: &str) -> Result<FontSpec, String> {
    let font = match id {
        "libertinus" => FontSpec {
            name: "Libertinus Serif",
            url: "https://cdn.jsdelivr.net/gh/typst/typst-assets@v0.13.1/files/fonts/LibertinusSerif-Regular.otf",
            css_fallback: "serif",
        },
        "literata" => FontSpec {
            name: "Literata",
            url: "https://fonts.gstatic.com/s/literata/v40/or3PQ6P12-iJxAIgLa78DkrbXsDgk0oVDaDPYLanFLHpPf2TbPa4F_Y.ttf",
            css_fallback: "serif",
        },
        "source-serif-4" => FontSpec {
            name: "Source Serif 4",
            url: "https://fonts.gstatic.com/s/sourceserif4/v14/vEFy2_tTDB4M7-auWDN0ahZJW3IX2ih5nk3AucvUHf6OAVIJmeUDygwjivBtrhw.ttf",
            css_fallback: "serif",
        },
        "atkinson-hyperlegible" => FontSpec {
            name: "Atkinson Hyperlegible",
            url: "https://fonts.gstatic.com/s/atkinsonhyperlegible/v12/9Bt73C1KxNDXMspQ1lPyU89-1h6ONRlW45G8WbcNcw.ttf",
            css_fallback: "sans-serif",
        },
        "space-grotesk" => FontSpec {
            name: "Space Grotesk",
            url: "https://fonts.gstatic.com/s/spacegrotesk/v22/V8mQoQDjQSkFtoMM3T6r8E7mF71Q-gOoraIAEj4PVksj.ttf",
            css_fallback: "sans-serif",
        },
        "dm-mono" => FontSpec {
            name: "DM Mono",
            url: "https://fonts.gstatic.com/s/dmmono/v16/aFTR7PB1QTsUX8KYvumzIYQ.ttf",
            css_fallback: "monospace",
        },
        _ => return Err(format!("unknown font family: {id}")),
    };
    Ok(font)
}

fn validate_font_size(font_size: f64) -> Result<(), String> {
    if !font_size.is_finite() || !(6.0..=36.0).contains(&font_size) {
        return Err("font_size must be between 6 and 36 points".to_owned());
    }
    Ok(())
}

fn validate_spacing(name: &str, spacing: f64) -> Result<(), String> {
    if !spacing.is_finite() || !(0.0..=12.0).contains(&spacing) {
        return Err(format!("{name} must be between 0 and 12 millimetres"));
    }
    Ok(())
}

fn page_spec(paper_size: &str, orientation: &str) -> Result<(f64, f64, &'static str), String> {
    let (width, height, css_paper) = match paper_size {
        "letter" => (215.9, 279.4, "letter"),
        "a3" => (297.0, 420.0, "A3"),
        "a4" => (210.0, 297.0, "A4"),
        "a5" => (148.0, 210.0, "A5"),
        _ => return Err(format!("unknown paper size: {paper_size}")),
    };
    match orientation {
        "portrait" => Ok((width, height, css_paper)),
        "landscape" => Ok((height, width, css_paper)),
        _ => Err(format!("unknown orientation: {orientation}")),
    }
}

fn escape_typst(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
