// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::collections::HashSet;

#[test]
fn test_empty_counter() {
    let counter = WordCounter::new();

    assert_eq!(counter.get(&String::from("Foobar")), 0);
    assert_eq!(counter.get("another"), 0);
    assert_eq!(counter.get(&String::from("")), 0);

    let counter = WordCounter::new();

    assert_eq!(counter.get(&String::from("Foobar")), 0);
    assert_eq!(counter.get("another"), 0);
    assert_eq!(counter.get(&String::new()), 0);

    assert_eq!(counter.total_count(), 0);
}

#[test]
fn test_counting() {
    let mut counter = WordCounter::new();

    counter.add("One");
    counter.add(&String::from("Two"));
    counter.add("Two");
    counter.add("Three");
    counter.add("Three");
    counter.add(&String::from("Three"));

    assert_eq!(counter.get("one"), 1);
    assert_eq!(counter.get("two"), 2);
    assert_eq!(counter.get(&String::from("three")), 3);
    assert_eq!(counter.get(&String::from("zero")), 0);

    assert_eq!(counter.total_count(), 6);
}

#[test]
fn test_display() {
    let mut counter = WordCounter::new();
    assert_eq!(counter.to_string(), "WordCounter, total count: 0\n");

    counter.add("One");
    assert_eq!(counter.to_string(), "WordCounter, total count: 1\none: 1\n");

    counter.add("Two"); counter.add("Two");
    assert_eq!(counter.to_string(), "WordCounter, total count: 3\ntwo: 2\none: 1\n");

    counter.add("Three"); counter.add("Three"); counter.add("Three");
    assert_eq!(counter.to_string(), "WordCounter, total count: 6\nthree: 3\ntwo: 2\none: 1\n");
}

#[test]
fn test_clean_line_removes_punctuation() {
    assert_eq!(clean_line("foo, bar, baz"), String::from("foo bar baz"));
    assert_eq!(clean_line("ала,  бала'ница"), String::from("ала  бала'ница"));
    assert_eq!(clean_line("-+/"), String::from("-"));
}

#[test]
fn test_clean_line_trims_the_input() {
    assert_eq!(clean_line(" foo  "), String::from("foo"));
    assert_eq!(clean_line("\tfoo"), String::from("foo"));
    assert_eq!(clean_line("  "), String::from(""));
    assert_eq!(clean_line(""), String::from(""));
}

#[test]
fn test_from_empty_str() {
    let source = String::from("");
    let counter = WordCounter::from_str(&source);
    let words: Vec<_> = counter.words().into_iter().map(|w| w.as_str()).collect();

    assert_eq!(words.len(), 0);
}

#[test]
fn test_from_str() {
    let source = String::from("Любов,  любов,   варен картоф.\nОбелих го, а той суров");
    let counter = WordCounter::from_str(&source);
    let words: Vec<_> = counter.words().into_iter().map(|w| w.as_str()).collect();

    assert_eq!(words, vec!["а", "варен", "го", "картоф", "любов", "обелих", "суров", "той"]);
}

#[test]
fn test_probability() {
    let spell_checker = SpellChecker::new("one one two two", "");
    assert_eq!(spell_checker.probability(&String::from("one")), 0.5);
    assert_eq!(spell_checker.probability(&String::from("two")), 0.5);
    assert_eq!(spell_checker.probability(&String::from("three")), 0.0);

    let spell_checker = SpellChecker::new("three four four four", "");
    assert_eq!(spell_checker.probability(&String::from("three")), 0.25);
    assert_eq!(spell_checker.probability(&String::from("four")), 0.75);
    assert_eq!(spell_checker.probability(&String::from("five")), 0.0);

    let spell_checker = SpellChecker::new("one", "");
    assert_eq!(spell_checker.probability(&String::from("one")), 1.0);
}

#[test]
fn test_edits1() {
    let edits = SpellChecker::new("", "влпу").edits1("три");

    // deletions
    assert!(edits.contains("ти"));
    assert!(edits.contains("тр"));
    assert!(edits.contains("ри"));
    assert!(!edits.contains("р"));

    // transposes
    assert!(edits.contains("тир"));
    assert!(edits.contains("рти"));
    assert!(!edits.contains("ирт"));

    // replaces
    assert!(edits.contains("тли"));
    assert!(edits.contains("тру"));
    assert!(!edits.contains("тлу"));

    // inserts
    assert!(edits.contains("трип"));
    assert!(edits.contains("втри"));
    assert!(!edits.contains("втрип"));
}

#[test]
fn test_edits2() {
    let edits = SpellChecker::new("", "авезийпсц").edits2("три");

    // deletions
    assert!(edits.contains("т"));
    assert!(edits.contains("р"));
    assert!(edits.contains("и"));
    assert!(!edits.contains("з"));

    // transposes
    assert!(edits.contains("итр"));

    // replaces
    assert!(edits.contains("спи"));
    assert!(edits.contains("таз"));
    assert!(!edits.contains("две"));

    // inserts
    assert!(edits.contains("втрий"));
    assert!(edits.contains("тирц"));
    assert!(!edits.contains("тирцц"));
}

#[test]
fn test_known_words() {
    let spell_checker = SpellChecker::new("one two three four", "abcdefghijklmnopqrstuvwxyz");
    let words: HashSet<_> = vec![
        String::from("one"),
        String::from("seven"),
        String::from("thre"),
        String::from("four")
    ].into_iter().collect();
    let mut known_words = spell_checker.known(&words);
    known_words.sort();

    assert_eq!(known_words, vec![&String::from("four"), &String::from("one")]);
}

#[test]
fn test_correction() {
    let spell_checker = SpellChecker::new("Любов, любов, варен картоф", "вюф");

    let word = String::from("либоф");
    assert_eq!(spell_checker.correction(&word), "любов");

    let word = String::from("картов");
    assert_eq!(spell_checker.correction(&word), "картоф");
}

#[test]
fn test_correction_normalizes_case() {
    let spell_checker = SpellChecker::new("Любов, любов, варен картоф", "вюф");

    let word = String::from("Либоф");
    assert_eq!(spell_checker.correction(&word), "любов");

    let word = String::from("Картов");
    assert_eq!(spell_checker.correction(&word), "картоф");
}

#[test]
fn test_correction_fails_to_produce_new_result() {
    let spell_checker = SpellChecker::new("Любов, любов, варен картоф", "");
    let word = String::from("Либоф");
    assert_eq!(spell_checker.correction(&word), "либоф");

    let spell_checker = SpellChecker::new("Любов, любов, варен картоф", "abcdefghijklmnopqrstuvwxyz");
    let word = String::from("Либоф");
    assert_eq!(spell_checker.correction(&word), "либоф");

    let spell_checker = SpellChecker::new("либофф", "ф");
    let word = String::from("Либоф");
    assert_eq!(spell_checker.correction(&word), "либофф");
}

#[test]
fn test_best_word_is_returned() {
    // Most probability:
    let spell_checker = SpellChecker::new("boat boot boot boot", "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(spell_checker.correction("beet"), "boot");

    let spell_checker = SpellChecker::new("boat boat boat boot", "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(spell_checker.correction("beet"), "boat");

    // One edit away, instead of two
    let spell_checker = SpellChecker::new("own pawns pawns pawns", "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(spell_checker.correction("pwn"), "own");
}
