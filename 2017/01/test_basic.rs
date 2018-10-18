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

#[test]
fn test_baba_comma_dqdo() {
    let t = TextInfo::new("Баба,дядо");

    assert_eq!(9, t.char_count());
    assert_eq!(8, t.alphabetic_count());
    assert_eq!(8, t.cyrillic_letter_count());
    assert_eq!(0, t.latin_letter_count());
    assert_eq!(2, t.word_count());
    assert_eq!(0, t.sentence_count());
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_baba_and_dqdo() {
    let t = TextInfo::new("Баба и дядо");

    assert_eq!(11, t.char_count());
    assert_eq!(9, t.alphabetic_count());
    assert_eq!(9, t.cyrillic_letter_count());
    assert_eq!(0, t.latin_letter_count());
    assert_eq!(3, t.word_count());
    assert_eq!(0, t.sentence_count());
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_funky_baba_dqdo() {
    let t = TextInfo::new("~*_1-баба-x-дядо-1_*~");

    assert_eq!(21, t.char_count());
    assert_eq!(9, t.alphabetic_count());
    assert_eq!(8, t.cyrillic_letter_count());
    assert_eq!(1, t.latin_letter_count());
    assert_eq!(3, t.word_count());
    assert_eq!(0, t.sentence_count());
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_funky_one_two_three() {
    let t = TextInfo::new("one, two, and three!");

    assert_eq!(20, t.char_count());
    assert_eq!(14, t.alphabetic_count());
    assert_eq!(0, t.cyrillic_letter_count());
    assert_eq!(14, t.latin_letter_count());
    assert_eq!(4, t.word_count());
    assert_eq!(1, t.sentence_count());
    assert_eq!("😮", t.emotion());
}

#[test]
fn test_funky_huh_whats_this() {
    let t = TextInfo::new("Huh? What's this??");

    assert_eq!(18, t.char_count());
    assert_eq!(12, t.alphabetic_count());
    assert_eq!(0, t.cyrillic_letter_count());
    assert_eq!(12, t.latin_letter_count());
    assert_eq!(4, t.word_count());
    assert_eq!(2, t.sentence_count());
    assert_eq!("🤔", t.emotion());
}


#[test]
fn test_funky_hmm() {
    let t = TextInfo::new("Hmm... Hm, Hm, Hmm.");

    assert_eq!(19, t.char_count());
    assert_eq!(10, t.alphabetic_count());
    assert_eq!(0, t.cyrillic_letter_count());
    assert_eq!(10, t.latin_letter_count());
    assert_eq!(4, t.word_count());
    assert_eq!(2, t.sentence_count());
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_last_example() {
    let t = TextInfo::new("Compiler error!! Oh, no... What just happened?");

    assert_eq!(46, t.char_count());
    assert_eq!(33, t.alphabetic_count());
    assert_eq!(0, t.cyrillic_letter_count());
    assert_eq!(33, t.latin_letter_count());
    assert_eq!(7, t.word_count());
    assert_eq!(3, t.sentence_count());
    assert_eq!("😐", t.emotion());
}

#[test]
fn test_0_senteces() {
    // by idea of Александър Иванов (https://github.com/alehander42)
    let t = TextInfo::new("?...!");

    assert_eq!(5, t.char_count());
    assert_eq!(0, t.alphabetic_count());
    assert_eq!(0, t.cyrillic_letter_count());
    assert_eq!(0, t.latin_letter_count());
    assert_eq!(0, t.word_count());
    assert_eq!(0, t.sentence_count());
    assert_eq!("😐", t.emotion());
}
