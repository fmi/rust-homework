// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

macro_rules! set {
    ($($item:expr),*) => {
        {
            let mut hash_set = ::std::collections::HashSet::new();
            $( hash_set.insert($item); );*
            hash_set
        }
    };
}

#[test]
fn test_extract_words_basic() {
    let words = extract_words("The <cat> is (in the) </bag>");
    assert_eq!(vec!["The", "cat", "is", "in", "the", "bag"], words);

    let text = String::from("The <cat> is (in the) </bag>");
    let words = extract_words(&text);
    assert_eq!(vec!["The", "cat", "is", "in", "the", "bag"], words);
}

#[test]
fn test_extract_words_extra() {
    let words = extract_words("Кво стаа, Пешо?");
    assert_eq!(vec!["Кво", "стаа", "Пешо"], words);

    let text = String::from("Кво стаа, Пешо?");
    let words = extract_words(&text);
    assert_eq!(vec!["Кво", "стаа", "Пешо"], words);

    let words = extract_words("   *-~spaces~-* in <<<places>>>   ");
    assert_eq!(vec!["spaces", "in", "places"], words);

    let words = extract_words("");
    assert_eq!(Vec::<String>::new(), words);
}

#[test]
fn test_search_word() {
    let mut index = TextIndex::new();
    index.push("one, two, three");
    index.push("two, neon");
    index.push("one/five/six");
    index.push("five, six, seven");

    assert_eq!(set!{"one/five/six", "one, two, three"}, index.search("one"));
    assert_eq!(set!{"five, six, seven", "one/five/six"}, index.search("six"));
}

#[test]
fn test_search_multiple_words() {
    let mut index = TextIndex::new();
    index.push("one, two");
    index.push("two, three");
    index.push("four, five");

    assert_eq!(set!{"four, five", "one, two"}, index.search("one + four"));
}

#[test]
fn test_search_special_cases() {
    let mut index = TextIndex::new();
    index.push("one, two");
    index.push("two, three");
    index.push("two, three");
    index.push("");

    assert_eq!(set!{"one, two", "two, three"}, index.search(" two, two "));
}
