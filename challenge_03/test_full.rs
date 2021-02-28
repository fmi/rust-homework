use bigint_display::*;

use std::str::FromStr;

#[test]
fn test_bigint_parsing() {
    assert!(Bigint::from_str("123").is_ok());
    assert!(Bigint::from_str("111").is_ok());
    assert!(Bigint::from_str("").is_ok());

    assert!(Bigint::from_str("foobar").is_err());
    assert!(Bigint::from_str(" ").is_err());
}

fn bigint(s: &str) -> Bigint {
    Bigint::from_str(s).unwrap()
}

#[test]
fn test_bigint_display() {
    assert_eq!(format!("{}", bigint("123")), "123");
    assert_ne!(format!("{}", bigint("111")), "222");
}

#[test]
fn test_bigint_delimited_display() {
    assert_eq!(format!("{}", bigint("12").delimited()),        "12");
    assert_eq!(format!("{}", bigint("123").delimited()),       "123");
    assert_eq!(format!("{}", bigint("1234").delimited()),      "1,234");
    assert_eq!(format!("{}", bigint("12345").delimited()),     "12,345");
    assert_eq!(format!("{}", bigint("123456").delimited()),    "123,456");
    assert_eq!(format!("{}", bigint("1234567").delimited()),   "1,234,567");
    assert_eq!(format!("{}", bigint("12345678").delimited()),  "12,345,678");
    assert_eq!(format!("{}", bigint("123456789").delimited()), "123,456,789");
}
