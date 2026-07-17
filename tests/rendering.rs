use word_deck::{render_html, render_text, render_typst};

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

    assert!(html.contains("&lt;danger&gt;"));
    assert!(html.contains("white-space: nowrap"));
    assert!(!html.contains("<danger>"));
}

#[test]
fn typst_export_escapes_quotes_and_uses_cutout_layout() {
    let typst = render_typst(&["don't".into(), "say \"go\"".into()]);

    assert!(typst.contains("say \\\"go\\\""));
    assert!(typst.contains("#box["));
    assert!(typst.contains("stroke: none"));
}
