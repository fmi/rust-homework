// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::panic::catch_unwind;

macro_rules! string_vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($($x:expr),+ $(,)?) => (
        vec![$($x),+].into_iter().map(String::from).collect::<Vec<_>>()
    );
}

#[test]
fn test_classic1() {
    assert_eq!(fizzbuzz(16), string_vec![
        "1",    "2",    "Fizz", "4",    "Buzz",
        "Fizz", "7",    "8",    "Fizz", "Buzz",
        "11",   "Fizz", "13",   "14",   "Fizzbuzz",
        "16"
    ]);

    assert_eq!(fizzbuzz(30).get(29), Some(&String::from("Fizzbuzz")));
}

#[test]
fn test_classic2() {
    let result   = fizzbuzz(300);
    let fizzbuzz = String::from("Fizzbuzz");

    assert_eq!(result.get(14), Some(&fizzbuzz));
    assert_eq!(result.get(29), Some(&fizzbuzz));
    assert_eq!(result.get(44), Some(&fizzbuzz));
}

#[test]
fn test_coefficients1() {
    assert_eq!(custom_buzz(16, 5, 7), string_vec![
        "1",  "2",    "3",  "4",    "Fizz",
        "6",  "Buzz", "8",  "9",    "Fizz",
        "11", "12",   "13", "Buzz", "Fizz",
        "16"
    ]);

    assert_eq!(custom_buzz(35, 5, 7).get(34), Some(&String::from("Fizzbuzz")));
    assert_eq!(custom_buzz(22, 2, 11).get(21), Some(&String::from("Fizzbuzz")));
}

#[test]
fn test_coefficients2() {
    let result   = custom_buzz(100, 3, 5);
    let fizzbuzz = String::from("Fizzbuzz");

    assert_eq!(result.get(14), Some(&fizzbuzz));
    assert_eq!(result.get(29), Some(&fizzbuzz));
    assert_eq!(result.get(44), Some(&fizzbuzz));
}

#[test]
fn test_coefficients_invalid() {
    assert!(catch_unwind(|| { custom_buzz(10, 3, 0); }).is_err());
    assert!(catch_unwind(|| { custom_buzz(10, 0, 5); }).is_err());
    assert!(catch_unwind(|| { custom_buzz(10, 3, 1); }).is_err());
    assert!(catch_unwind(|| { custom_buzz(10, 1, 5); }).is_err());
}

#[test]
fn test_struct_basic() {
    let fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Физ"),
            String::from("Бъз"),
            String::from("Физбъз")
        ],
    };
    assert_eq!(fizzbuzzer.take(16), string_vec![
        "1",   "2",   "Физ", "4",   "Бъз",
        "Физ", "7",   "8",   "Физ", "Бъз",
        "11",  "Физ", "13",  "14",  "Физбъз",
        "16"
    ]);
}

#[test]
fn test_struct_invalid() {
    macro_rules! fizzbuzzer {
        ($k1:expr, $k2:expr) => {
            FizzBuzzer {
                k1: $k1, k2: $k2,
                labels: [String::from("Fizz"), String::from("Buzz"), String::from("Fizzbuzz")],
            }.take(10)
        }
    }

    assert!(catch_unwind(|| { fizzbuzzer!(3, 0); }).is_err());
    assert!(catch_unwind(|| { fizzbuzzer!(0, 5); }).is_err());
    assert!(catch_unwind(|| { fizzbuzzer!(3, 1); }).is_err());
    assert!(catch_unwind(|| { fizzbuzzer!(1, 5); }).is_err());
}

#[test]
fn test_change_label_basic() {
    let mut fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz")
        ],
    };

    let result = fizzbuzzer.take(16);
    assert_eq!(result.get(2),  Some(&String::from("Fizz")));
    assert_eq!(result.get(4),  Some(&String::from("Buzz")));
    assert_eq!(result.get(14), Some(&String::from("Fizzbuzz")));

    fizzbuzzer.change_label(1, &String::from("Бъз"));
    let result = fizzbuzzer.take(16);
    assert_eq!(result.get(2),  Some(&String::from("Fizz")));
    assert_eq!(result.get(4),  Some(&String::from("Бъз")));
    assert_eq!(result.get(14), Some(&String::from("Fizzbuzz")));
}

#[test]
#[should_panic]
fn test_change_label_invalid() {
    let mut fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz")
        ],
    };

    fizzbuzzer.change_label(3, &String::new());
}


#[test]
fn test_zeroes() {
    assert_eq!(fizzbuzz(0), Vec::<String>::new());
    assert_eq!(custom_buzz(0, 11, 13), Vec::<String>::new());

    let fizzbuzzer = FizzBuzzer {
        k1: 7,
        k2: 17,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz")
        ],
    };
    assert_eq!(fizzbuzzer.take(0), Vec::<String>::new());
}
