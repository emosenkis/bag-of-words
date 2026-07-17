use word_deck::{embedded_corpus, generate_json};

#[test]
fn embeds_frequency_only_asimov_and_fiction_corpora() {
    assert!(embedded_corpus("asimov").unwrap().frequency("the").unwrap() > 100_000);
    assert!(
        embedded_corpus("fiction-xlsx")
            .unwrap()
            .frequency("the")
            .unwrap()
            > 1_000_000
    );
    assert!(embedded_corpus("unknown").is_err());
}

#[test]
fn json_api_returns_the_requested_export() {
    let response = generate_json(r#"{"corpus":"asimov","count":40,"palette_size":20,"required_head":10,"seed":3,"format":"txt"}"#).unwrap();

    assert_eq!(response.format, "txt");
    assert_eq!(response.cards.len(), 40);
    assert!(response.content.lines().count() == 40);
}
