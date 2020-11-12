// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::borrow::Cow;

macro_rules! assert_match {
    ($expr:expr, $pat:pat) => {
        if let $pat = $expr {
            // all good
        } else {
            assert!(false, "Expression {:?} does not match the pattern {:?}", $expr, stringify!($pat));
        }
    }
}

#[test]
fn test_basic() {
    let fizzbuzzer = FizzBuzzer::new([
        String::from("Fizz"),
        String::from("Buzz"),
        String::from("Fizzbuzz"),
    ]);
    let items: Vec<_> = fizzbuzzer.iter().take(15).collect();

    assert_eq!(items, [
        "1", "2", "Fizz", "4", "Buzz",
        "Fizz", "7", "8", "Fizz", "Buzz",
        "11", "Fizz", "13", "14", "Fizzbuzz"
    ]);
}

#[test]
fn test_cow() {
    let fizzbuzzer = FizzBuzzer::new([
        String::from("Fizz"),
        String::from("Buzz"),
        String::from("Fizzbuzz"),
    ]);
    let mut iter = fizzbuzzer.iter();

    assert_match!(iter.next(), Some(Cow::Owned(_)));    // "1"
    assert_match!(iter.next(), Some(Cow::Owned(_)));    // "2"
    assert_match!(iter.next(), Some(Cow::Borrowed(_))); // "Fizz"
}

#[test]
fn test_labels() {
    let fizzbuzzer = FizzBuzzer::new([
        String::from("Four"),
        String::from("Seasons"),
        String::from("Total Landscaping"),
    ]);
    let items: Vec<_> = fizzbuzzer.iter().take(15).collect();

    assert_eq!(items, [
        "1", "2", "Four", "4", "Seasons",
        "Four", "7", "8", "Four", "Seasons",
        "11", "Four", "13", "14", "Total Landscaping"
    ]);
}
