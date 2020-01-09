// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::collections::HashSet;

#[test]
fn test_basic() {
    let line = String::from(" Foo Bar ");
    assert_eq!("Foo Bar", clean_line(&line));

    let mut counter = WordCounter::new();

    assert_eq!(counter.words(), WordCounter::from_str("").words());
    assert_eq!(counter.get("word"), 0);
    assert_eq!(counter.total_count(), 0);
    assert_eq!(format!("{}", counter), "WordCounter, total count: 0\n");

    counter.add("word");

    let spell_checker = SpellChecker::new("foo bar", ALPHABET_EN);
    assert_eq!(spell_checker.correction("foo"), "foo");
    assert!(spell_checker.probability("foo") > 0.0);
    assert!(spell_checker.known(&HashSet::new()).len() == 0);
    assert!(spell_checker.candidates("foo").len() > 0);
    assert!(spell_checker.edits1("foo").len() > 0);
    assert!(spell_checker.edits2("foo").len() > 0);
}
