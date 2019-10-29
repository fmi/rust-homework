// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:
use solution::*;

#[test]
fn test_basic() {
    assert_eq!(decimal("345"), Some(3));
    assert_eq!(hex("345"), Some(0xc));

    assert_eq!(octal("1"), Some(1));
    assert_eq!(binary("1"), Some(1));

    let num = String::from("1");
    assert_eq!(binary(&num[..]), Some(1));
}
