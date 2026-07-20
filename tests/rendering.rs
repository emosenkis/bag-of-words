use word_deck::{render_html, render_html_for_page, render_text, render_typst};

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
    let typst = render_typst(&["don't".into(), "say \"go\"".into()]);

    assert!(typst.contains("say \\\"go\\\""));
    assert!(typst.contains("words.sorted(key: word => (measure(text(word)).width, word))"));
    assert!(typst.contains("col * row_count + row"));
    assert!(typst.contains("box[#sorted.at(index)]"));
    assert!(typst.contains("stroke: none"));
}
