use word_deck::{Corpus, DeckConfig, generate_deck};

#[test]
fn parses_frequency_only_tsv() {
    let corpus = Corpus::from_tsv("word\tfrequency\nthe\t100\ndragon\t9\n").unwrap();

    assert_eq!(corpus.frequency("the"), Some(100));
    assert_eq!(corpus.frequency("dragon"), Some(9));
}

#[test]
fn deck_contains_the_required_frequency_head() {
    let corpus =
        Corpus::from_tsv("word\tfrequency\nthe\t100\nand\t80\ndragon\t9\ncastle\t7\n").unwrap();
    let config = DeckConfig {
        count: 12,
        palette_size: 4,
        required_head: 2,
        seed: 7,
    };

    let deck = generate_deck(&corpus, &config).unwrap();

    assert_eq!(deck.len(), 12);
    assert!(deck.contains(&"the".to_owned()));
    assert!(deck.contains(&"and".to_owned()));
}

#[test]
fn deck_is_deterministic_for_a_seed() {
    let corpus =
        Corpus::from_tsv("word\tfrequency\nthe\t100\nand\t80\ndragon\t9\ncastle\t7\nmagic\t5\n")
            .unwrap();
    let config = DeckConfig {
        count: 20,
        palette_size: 4,
        required_head: 2,
        seed: 99,
    };

    assert_eq!(
        generate_deck(&corpus, &config).unwrap(),
        generate_deck(&corpus, &config).unwrap()
    );
}
