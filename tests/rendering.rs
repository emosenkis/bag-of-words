use word_deck::{render_html, render_html_for_page, render_text, render_typst_for_page_with_font};

#[test]
fn text_export_has_one_card_per_line() {
    assert_eq!(
        render_text(&["dragon".into(), "don't".into()]),
        "dragon\ndon't\n"
    );
}

#[test]
fn html_export_escapes_cards_and_keeps_affixes_unbroken() {
    let html = render_html(&["<danger>".into(), "re-".into()]);

    assert!(html.contains("element.textContent = card.word"));
    assert!(html.contains("white-space: nowrap"));
    assert!(html.contains("[\"<danger>\",\"re-\"]"));
}

#[test]
fn html_export_contains_a_measured_page_packer_for_the_selected_paper() {
    let html = render_html_for_page(&["dragon".into(), "do".into()], "a4", "landscape").unwrap();

    assert!(html.contains("@page { size: A4 landscape"));
    assert!(html.contains("CanvasRenderingContext2D"));
    assert!(html.contains("measured.sort"));
    assert!(html.contains("rows = Math.max"));
}

#[test]
fn typst_export_escapes_quotes_and_uses_cutout_layout() {
    let typst = render_typst_for_page_with_font(
        &["don't".into(), "say \"go\"".into()],
        "a4",
        "portrait",
        16.0,
    )
    .unwrap();

    assert!(typst.contains("say \\\"go\\\""));
    assert!(typst.contains("sampled.sorted().sorted(key: word => measure(text(word)).width)"));
    assert!(typst.contains("#set text(font: \"Libertinus Serif\", size: 16pt)"));
    assert!(typst.contains("let row-height = measure(box(inset: (y: 2.4mm))[Ag]).height"));
    assert!(typst.contains("let rows-per-column = calc.floor(printable-height / row-height)"));
    assert!(typst.contains("let printable-width = 202mm"));
    assert!(typst.contains("let render-page(columns)"));
    assert!(typst.contains("pages.push(pagebreak())"));
}
