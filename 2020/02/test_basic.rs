// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

use std::str::FromStr;

fn bigint(s: &str) -> Bigint {
    Bigint::from_str(s).unwrap()
}

#[test]
fn test_basic() {
    assert_eq!(Bigint::new(), bigint("0"));
    assert!(Bigint::from_str("foobar").is_err());

    assert!(bigint("1").is_positive());
    assert!(bigint("-1").is_negative());

    assert_eq!(bigint("123") + bigint("456"), bigint("579"));
    assert_eq!(bigint("579") - bigint("456"), bigint("123"));

    assert!(bigint("123") > bigint("122"));
}
