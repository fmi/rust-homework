// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution as digital_root;

#[test]
fn test_decimal_basic() {
    assert_eq!(digital_root::decimal("345"), Some(3));

    for n in 0..=9 {
        assert_eq!(digital_root::decimal(&n.to_string()), Some(n));
    }

    assert_eq!(digital_root::decimal("10"), Some(1));
}

#[test]
fn test_hex_basic() {
    assert_eq!(digital_root::hex("345"), Some(0xc));
    assert_eq!(digital_root::hex("7b"), Some(0x3));

    for n in 0..=9 {
        assert_eq!(digital_root::hex(&n.to_string()), Some(n));
    }
    assert_eq!(digital_root::hex("a"), Some(0xa));
    assert_eq!(digital_root::hex("d"), Some(0xd));
    assert_eq!(digital_root::hex("f"), Some(0xf));

    assert_eq!(digital_root::hex("10"), Some(0x1));
}

#[test]
fn test_octal_basic() {
    assert_eq!(digital_root::octal("345"), Some(0o5));

    for n in 0..=7 {
        assert_eq!(digital_root::octal(&n.to_string()), Some(n));
    }
    assert_eq!(digital_root::octal("10"), Some(0o1));
}

#[test]
fn test_binary() {
    assert_eq!(digital_root::binary("101"), Some(1));
    assert_eq!(digital_root::binary("111"), Some(1));
    assert_eq!(digital_root::binary("100"), Some(1));
    assert_eq!(digital_root::binary("110"), Some(1));

    assert_eq!(digital_root::binary("1100101001010"), Some(1));
}

#[test]
fn test_zeroes() {
    assert_eq!(digital_root::decimal("0"), Some(0));
    assert_eq!(digital_root::hex("0"), Some(0));
    assert_eq!(digital_root::octal("0"), Some(0));
    assert_eq!(digital_root::binary("0"), Some(0));
}

#[test]
fn test_invalid() {
    assert_eq!(digital_root::decimal("f"), None);
    assert_eq!(digital_root::hex("g"), None);
    assert_eq!(digital_root::octal("8"), None);
    assert_eq!(digital_root::binary("2"), None);

    assert_eq!(digital_root::decimal("баба"), None);
    assert_eq!(digital_root::hex("баба"), None);
    assert_eq!(digital_root::octal("баба"), None);
    assert_eq!(digital_root::binary("баба"), None);
}
