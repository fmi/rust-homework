// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

#[test]
fn test_validity() {
    assert!(CodeIdentifier::new("some-var").is_none());
    assert!(CodeIdentifier::new("some var").is_none());
    assert!(CodeIdentifier::new("+_+").is_none());
    assert!(CodeIdentifier::new("❤️_❤️").is_none());
    assert!(CodeIdentifier::new("123abc").is_none());
    assert!(CodeIdentifier::new("a+b=c").is_none());

    assert!(CodeIdentifier::new("abc123").is_some());
    assert!(CodeIdentifier::new("some_var").is_some());
    assert!(CodeIdentifier::new(" some_var ").is_some());
    assert!(CodeIdentifier::new("someVar").is_some());
    assert!(CodeIdentifier::new("SomeVar").is_some());
}

#[test]
fn test_both_static_and_dynamic_strings() {
    assert!(CodeIdentifier::new("some_var").is_some());

    let some_var = String::from("some_var");
    assert!(CodeIdentifier::new(&some_var).is_some());

    let some_var = "some_var";
    assert!(CodeIdentifier::new(&some_var).is_some());

    assert!(CodeIdentifier::new(&String::from("some_var")).is_some());
}

#[test]
fn test_camelcase_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.camelcase(), "someVar");
}

#[test]
fn test_underscore_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.underscore(), "some_var");
}

#[test]
fn test_screaming_snakecase_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.screaming_snakecase(), "SOME_VAR");
}

#[test]
fn test_titlecase_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.titlecase(), "SomeVar");
}

#[test]
fn test_kebabcase_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.kebabcase(), "some-var");
}

#[test]
fn test_normalize_case1() {
    let code_identifier = CodeIdentifier::new("Some_Var").unwrap();
    assert_eq!(code_identifier.underscore(), "some_var");

    let code_identifier = CodeIdentifier::new("SomeVar").unwrap();
    assert_eq!(code_identifier.underscore(), "somevar");

    let code_identifier = CodeIdentifier::new("SOME_VAR").unwrap();
    assert_eq!(code_identifier.underscore(), "some_var");
}

#[test]
fn test_normalize_case2() {
    let code_identifier = CodeIdentifier::new("Some_Var").unwrap();
    assert_eq!(code_identifier.camelcase(), "someVar");

    let code_identifier = CodeIdentifier::new("SomeVar").unwrap();
    assert_eq!(code_identifier.screaming_snakecase(), "SOMEVAR");
    assert_eq!(code_identifier.camelcase(), "somevar");

    let code_identifier = CodeIdentifier::new("SOME_VAR").unwrap();
    assert_eq!(code_identifier.screaming_snakecase(), "SOME_VAR");
}

#[test]
fn test_cyrillic1() {
    let code_identifier = CodeIdentifier::new("що_стана").unwrap();
    assert_eq!(code_identifier.underscore(), "що_стана");
    assert_eq!(code_identifier.screaming_snakecase(), "ЩО_СТАНА");
    assert_eq!(code_identifier.camelcase(), "щоСтана");
    assert_eq!(code_identifier.titlecase(), "ЩоСтана");
    assert_eq!(code_identifier.kebabcase(), "що-стана");
}

#[test]
fn test_multibyte_uppercase() {
    let code_identifier = CodeIdentifier::new("some_ßpecial_case").unwrap();
    assert_eq!(code_identifier.camelcase(), "someSSpecialCase");
    assert_eq!(code_identifier.screaming_snakecase(), "SOME_SSPECIAL_CASE");
}

#[test]
fn test_whitespace() {
    let code_identifier = CodeIdentifier::new("  some_var\n").unwrap();
    assert_eq!(code_identifier.underscore(), "some_var");

    let code_identifier = CodeIdentifier::new("\tone_two_three  ").unwrap();
    assert_eq!(code_identifier.camelcase(), "oneTwoThree");
}

#[test]
fn test_digits1() {
    let code_identifier = CodeIdentifier::new("some_var_123").unwrap();
    assert_eq!(code_identifier.camelcase(), "someVar123");
    assert_eq!(code_identifier.underscore(), "some_var_123");

    let code_identifier = CodeIdentifier::new("some_123_var").unwrap();
    assert_eq!(code_identifier.camelcase(), "some123Var");
    assert_eq!(code_identifier.underscore(), "some_123_var");
}

#[test]
fn test_digits2() {
    let code_identifier = CodeIdentifier::new("some_1_2_var").unwrap();
    assert_eq!(code_identifier.camelcase(), "some12Var");
    assert_eq!(code_identifier.underscore(), "some_1_2_var");

    let code_identifier = CodeIdentifier::new("because_7_8_9").unwrap();
    assert_eq!(code_identifier.camelcase(), "because789");
    assert_eq!(code_identifier.underscore(), "because_7_8_9");
}

#[test]
fn test_digits3() {
    let code_identifier = CodeIdentifier::new("some_٣_var").unwrap();
    assert_eq!(code_identifier.camelcase(), "some٣Var");
    assert_eq!(code_identifier.underscore(), "some_٣_var");
}
