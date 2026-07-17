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
    format: String,
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
        "html" => render_html(&cards),
        "typst" => render_typst(&cards),
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
    let cards = cards
        .iter()
        .map(|card| format!("<span class=\"card\">{}</span>", escape_html(card)))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><style>\
         @page {{ margin: 4mm; }} body {{ font: 14pt sans-serif; }} .deck {{ columns: 8; gap: 3mm; }}\
         .card {{ display: block; white-space: nowrap; line-height: 1.5; text-align: center; }}\
         </style></head><body><main class=\"deck\">{cards}</main></body></html>"
    )
}

pub fn render_typst(cards: &[String]) -> String {
    let rows = cards
        .iter()
        .map(|card| format!("  #box[{}]", escape_typst(card)))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        "#set page(margin: 4mm)\n#set text(size: 14pt)\n#table(\n  columns: 8,\n  stroke: none,\n  inset: (x: 1.5mm, y: 2.4mm),\n{rows},\n)\n"
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_typst(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
