#!/usr/bin/env python3
"""Build a frequency-only TSV from Wiktionary's ranked English Wikipedia list."""

from __future__ import annotations

import re
from pathlib import Path
from urllib.request import Request, urlopen

SOURCE = "https://en.wiktionary.org/w/index.php?title=Wiktionary:Frequency_lists/English/Wikipedia_(2016)&action=raw"
LINK = re.compile(r"^\[\[([^#\]|]+)#English\|[^]]+\]\]$", re.MULTILINE)
WORD = re.compile(r"[a-z]+(?:'[a-z]+)?$")
OUTPUT = Path(__file__).resolve().parents[1] / "assets" / "wikipedia.tsv"


def main() -> None:
    request = Request(SOURCE, headers={"User-Agent": "BagOfWordsCorpusBuilder/1.0"})
    with urlopen(request) as response:  # nosec B310: fixed public source URL
        raw = response.read().decode("utf-8")
    seen: set[str] = set()
    words: list[str] = []
    for raw_word in LINK.findall(raw):
        word = raw_word.lower()
        if WORD.fullmatch(word) and word not in seen:
            seen.add(word)
            words.append(word)
    if len(words) < 8_000:
        raise SystemExit(f"Expected roughly 10,000 ranked words, found {len(words)}")
    lines = ["word\tfrequency"]
    for rank, word in enumerate(words, start=1):
        # The public source provides ranks, not counts. A Zipf-like conversion
        # preserves its ordering while making it usable by the sampler.
        lines.append(f"{word}\t{max(1, round(10_000_000 / rank))}")
    OUTPUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Wrote {len(words)} ranked English Wikipedia words to {OUTPUT}")


if __name__ == "__main__":
    main()
