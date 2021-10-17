// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::str::FromStr;

fn bigint(s: &str) -> Bigint {
    Bigint::from_str(s).unwrap()
}

#[test]
fn test_bigint_construction() {
    assert_eq!(Bigint::new(), bigint("0"));
    assert_eq!(Bigint::new(), bigint(""));
}

#[test]
fn test_bigint_zero_sign() {
    let zero = bigint("0");
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());

    let positive_zero = bigint("+0");
    assert!(!positive_zero.is_positive());
    assert!(!positive_zero.is_negative());

    let negative_zero = bigint("-0");
    assert!(!negative_zero.is_positive());
    assert!(!negative_zero.is_negative());
}

#[test]
fn test_bigint_nonzero_sign() {
    let number = bigint("123");
    assert!(number.is_positive());
    assert!(!number.is_negative());

    let positive_number = bigint("+123");
    assert!(positive_number.is_positive());
    assert!(!positive_number.is_negative());

    let negative_number = bigint("-123");
    assert!(!negative_number.is_positive());
    assert!(negative_number.is_negative());
}

#[test]
fn test_invalid_string() {
    assert!(Bigint::from_str(" 123").is_err());
    assert!(Bigint::from_str("123 ").is_err());
    assert!(Bigint::from_str("baba").is_err());
    assert!(Bigint::from_str("123b").is_err());
    assert!(Bigint::from_str("b123").is_err());
    assert!(Bigint::from_str("12 345").is_err());
    assert!(Bigint::from_str("кирилица").is_err());
    assert!(Bigint::from_str("123к345").is_err());
}

#[test]
fn test_parsing_with_and_without_sign() {
    assert_eq!(bigint("123"), bigint("+123"));
    assert_eq!(bigint("456"), bigint("+456"));
    assert_eq!(bigint("12345678901234567890"), bigint("+12345678901234567890"));
}

#[test]
fn test_parsing_with_leading_zeroes() {
    assert_eq!(bigint("123"), bigint("0123"));
    assert_eq!(bigint("456"), bigint("0000456"));
    assert_eq!(bigint("0000"), bigint("0"));
}

#[test]
fn test_sum_1_basic() {
    assert_eq!(bigint("1")   + bigint("2"),   bigint("3"));
    assert_eq!(bigint("111") + bigint("222"), bigint("333"));

    assert_eq!(
        bigint("11111111111111111111") +
        bigint("22222222222222222222"),
        bigint("33333333333333333333")
    );
}

#[test]
fn test_sum_2_different_lengths() {
    assert_eq!(bigint("12")   + bigint("12000"), bigint("12012"));
    assert_eq!(bigint("1")    + bigint("123"),   bigint("124"));
    assert_eq!(bigint("111")  + bigint("0"),     bigint("111"));
    assert_eq!(bigint("0")    + bigint("444"),   bigint("444"));
    assert_eq!(bigint("+123") + bigint("+456"),  bigint("579"));
}

#[test]
fn test_sum_3_overflow() {
    assert_eq!(bigint("999") + bigint("1"),   bigint("1000"));
    assert_eq!(bigint("511") + bigint("599"), bigint("1110"));

    assert_eq!(bigint("-999") + bigint("-1"),   bigint("-1000"));
    assert_eq!(bigint("-511") + bigint("-599"), bigint("-1110"));

    assert_eq!(
        bigint("99999999999999999999") +
        bigint("1"),
        bigint("100000000000000000000")
    );
}

#[test]
fn test_sum_4_negative() {
    assert_eq!(bigint("-123") + bigint("-456"),   bigint("-579"));
    assert_eq!(bigint("-12")  + bigint("-12000"), bigint("-12012"));
    assert_eq!(bigint("-1")   + bigint("-123"),   bigint("-124"));
    assert_eq!(bigint("-111") + bigint("-0"),     bigint("-111"));
    assert_eq!(bigint("-0")   + bigint("-444"),   bigint("-444"));

    assert_eq!(bigint("-123") + bigint("456"), bigint("333"));
    assert_eq!(bigint("123")  + bigint("-456"), bigint("-333"));
    assert_eq!(bigint("456")  + bigint("-123"), bigint("333"));
    assert_eq!(bigint("-456") + bigint("123"), bigint("-333"));
}

#[test]
fn test_neutralization() {
    assert_eq!(bigint("-123") + bigint("123"), bigint("0"));
    assert_eq!(bigint("123") + bigint("-123"), bigint("0"));
    assert_eq!(bigint("-0") + bigint("+0"), bigint("0"));
    assert_eq!(bigint("+0") + bigint("-0"), bigint("0"));

    assert_eq!(bigint("123")  - bigint("123"),  bigint("0"));
    assert_eq!(bigint("-123") - bigint("-123"), bigint("0"));
    assert_eq!(bigint("-0")   - bigint("-0"),   bigint("0"));
    assert_eq!(bigint("+0")   - bigint("+0"),   bigint("0"));
}

#[test]
fn test_sub_1_basic() {
    assert_eq!(bigint("567") - bigint("123"), bigint("444"));
    assert_eq!(bigint("123") - bigint("567"), bigint("-444"));

    assert_eq!(
        bigint("33333333333333333333") -
        bigint("22222222222222222222"),
        bigint("11111111111111111111")
    );
}

#[test]
fn test_sub_2_diferent_lengths() {
    assert_eq!(bigint("1100") - bigint("100"), bigint("1000"));
    assert_eq!(bigint("1567") - bigint("123"), bigint("1444"));
}

#[test]
fn test_sub_3_carry() {
    assert_eq!(bigint("1000") - bigint("1"),    bigint("999"));
    assert_eq!(bigint("1110") - bigint("599"),  bigint("511"));
    assert_eq!(bigint("123")  - bigint("1567"), bigint("-1444"));

    assert_eq!(
        bigint("100000000000000000000") -
        bigint("1"),
        bigint("99999999999999999999")
    );
}

#[test]
fn test_comparison() {
    assert!(bigint("1")  > bigint("-1"));
    assert!(bigint("-1") > bigint("-2"));
    assert!(bigint("-1") > bigint("-10"));
    assert!(bigint("0")  > bigint("-1"));

    assert!(bigint("123") > bigint("12"));
    assert!(bigint("123") > bigint("122"));

    assert!(!(bigint("123") > bigint("123")));
    assert!(bigint("123") >= bigint("123"));

    assert!(bigint("1000") > bigint("999"));
    assert!(bigint("999") < bigint("1000"));
}
