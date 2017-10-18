// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::TextInfo;

#[test]
fn test_basic() {
    let t = TextInfo::new("abc. 123. абв.");

    assert_eq!(14, t.char_count());
    assert_eq!(6, t.alphabetic_count());
    assert_eq!(3, t.cyrillic_letter_count());
    assert_eq!(3, t.latin_letter_count());
    assert_eq!(2, t.word_count());
    assert_eq!(3, t.sentence_count());
    assert_eq!("😐", t.emotion());
}
