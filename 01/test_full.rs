// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::TextInfo;

#[test]
fn test_char_count() {
    let t = TextInfo::new("123456789");
    assert_eq!(9, t.char_count());
}

#[test]
fn test_unicode_char_count() {
    let t = TextInfo::new("abcd-абвг");
    assert_eq!(4 + 1 + 4, t.char_count());
}

#[test]
fn test_alpha_count() {
    let t = TextInfo::new(" AbCd-АбВг- ");
    assert_eq!(4 + 4, t.alphabetic_count());
}

#[test]
fn test_alpha_count_2() {
    let t = TextInfo::new("1. one, 2) две, 3-> three");
    assert_eq!(3 + 3 + 5, t.alphabetic_count());
}

#[test]
fn test_cyrillic_letter_count() {
    let t = TextInfo::new("юя-1-abc-АбВг");
    assert_eq!(2 + 4, t.cyrillic_letter_count());
}

#[test]
fn test_latin_letter_count() {
    let t = TextInfo::new("abc-абвг-XYZ");
    assert_eq!(3 + 3, t.latin_letter_count());
}

#[test]
fn test_word_count() {
    let t = TextInfo::new("  На баба ми хвърчилото! ");
    assert_eq!(4, t.word_count());
}

#[test]
fn test_word_count_2() {
    let t = TextInfo::new("Здравей, как си, приятелю? Как, добре ли я караш?");
    assert_eq!(9, t.word_count());
}

#[test]
fn test_word_count_3() {
    let t = TextInfo::new("Да видим кой е по-по-най!");
    assert_eq!(7, t.word_count());
}

#[test]
fn test_empty_string() {
    let t = TextInfo::new("");
    assert_eq!(0, t.char_count());
    assert_eq!(0, t.alphabetic_count());
    assert_eq!(0, t.latin_letter_count());
    assert_eq!(0, t.cyrillic_letter_count());
    assert_eq!(0, t.word_count());
    assert_eq!(0, t.sentence_count());
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_sentence_count() {
    let t = TextInfo::new("Баба ти само Rust е писала 1 време. Assembly language, uphill 2 ways! Кви са тия несериозни неща? Амаха.");
    assert_eq!(4, t.sentence_count());
}

#[test]
fn test_sentence_count_2() {
    let t = TextInfo::new("Да видим сега: следват ли студентите указания; Ще игнорират ли -- тази -- пунктуация?");
    assert_eq!(1, t.sentence_count());
}

#[test]
fn test_triple_dots_count() {
    let t = TextInfo::new("Somewhere... Over the rainbow...");
    assert_eq!(2, t.sentence_count());
}

#[test]
fn test_emotions() {
    let t = TextInfo::new("Explosion! WOOO!");
    assert_eq!("😮", t.emotion());

    let t = TextInfo::new("Wtf? What do?");
    assert_eq!("🤔", t.emotion());

    let t = TextInfo::new("Bang! Wait. What?");
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_emotions_repeated_punctuation() {
    let t = TextInfo::new("Explosion!! WOO! hey????");
    assert_eq!("😮", t.emotion());

    let t = TextInfo::new("Wtf?? Wat? What do!!!!");
    assert_eq!("🤔", t.emotion());

    let t = TextInfo::new("Bang! Wait... What?");
    assert_eq!("😐", t.emotion());
}
